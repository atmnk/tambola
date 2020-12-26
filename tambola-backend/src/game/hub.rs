use tokio::sync::RwLock;
use std::collections::HashMap;
use uuid::Uuid;
use crate::game::instance::GameInstance;
use tambola_lib::game::{GameSnapshot, User, UserType};
use crate::server::{UserWSSHandle, UserWSRHandle};
use tambola_lib::game::proto::{Input,Output};

pub struct Hub{
    pub games:RwLock<HashMap<Uuid,RwLock<GameInstance>>>
}
impl Hub{
    pub async fn host_new_game(&self,host_name:String)->(Uuid,User){
        let game_id = Uuid::new_v4();
        let host_user = User::new(host_name,UserType::Host);
        let mut users = HashMap::new();
        users.insert(host_user.id.clone(),RwLock::new(host_user.clone()));
        let game_instance = GameInstance{
            id:game_id.clone(),
            host:host_user.id.clone(),
            users:RwLock::new(users),
            snapshot:RwLock::new(GameSnapshot::blank()),
            user_handles:RwLock::new(HashMap::new()),
            announcements:RwLock::new(vec![])
        };
        let mut games = self.games.write().await;
        games.insert(game_id.clone(),RwLock::new(game_instance));
        (game_id.clone(),host_user)
    }
    pub async fn start_or_resume(&self,user_s_handle:UserWSSHandle,mut user_r_handle:UserWSRHandle){
        let mut exit = false;
        let mut insert_handle = false;
        let mut opt_user_id = Option::None;
        let mut opt_game_id = Option::None;
        let mut user_name = "".to_string();
        let mut outputs = vec![];
        while !exit {
            let message = user_r_handle.get_message().await;


            match message {
                Input::HostNewGame(hngi)=>{
                    let (new_game_id,new_user) = self.host_new_game(hngi.name.clone()).await;
                    insert_handle = true;
                    outputs.push(Output::new_game_hosted(new_game_id.clone(),new_user.clone()));
                    opt_game_id = Option::Some(new_game_id.clone());
                    opt_user_id = Option::Some(new_user.id.clone());
                    user_name = new_user.name.clone();
                    exit = true;
                },
                Input::Reconnect(rci)=>{
                    let games = self.games.read().await;
                    if let Some(rw_game) = games.get(&rci.game_id) {
                        let game = rw_game.write().await;
                        let opt_user = game.reconnect(rci.user_id.clone()).await;

                        if let Some(user) = opt_user {
                            let gss = game.snapshot.read().await;
                            let anc = game.announcements.read().await;
                            insert_handle = true;
                            user_name = user.name.clone();
                            opt_game_id = Some(game.id.clone());
                            opt_user_id = Some(user.id.clone());
                            outputs.push(Output::new_reconnected_to_game(user.clone(),gss.clone(),anc.clone()));
                            exit = true;
                        }

                    }
                },
                Input::ConnectMeAs(cmai)=>{
                    let games = self.games.read().await;
                    if let Some(rw_game) = games.get(&cmai.game) {
                        let game = rw_game.read().await;
                        let gss= game.snapshot.read().await;
                        let anc = game.announcements.read().await;
                        let user = game.join(cmai.name).await;
                        outputs.push(Output::connected_to_game(user.clone(),gss.clone(),anc.clone()));
                        outputs.push(Output::user_joined(user.name.clone()));
                        opt_game_id = Option::Some(cmai.game.clone());
                        opt_user_id = Option::Some(user.id.clone());
                        user_name = user.name.clone();
                        insert_handle = true;
                        exit = true;
                    } else {
                        outputs.push(Output::connect_me_failed("Game not found".to_string()));
                    }

                }
                _=>{}
            }
        };
        if let Some(game_id) = opt_game_id {
            let games = self.games.read().await;
            let rw_game = games.get(&game_id).unwrap();
            let game = rw_game.read().await;
            let handle = game.send_messages_properly(user_s_handle,outputs).await;
            if let Some(user_id) = opt_user_id {
                if insert_handle {
                    game.insert_handle(user_id.clone(), handle).await
                }
            }
        }
        if let Some(game_id) = opt_game_id {
            if let Some(user_id) = opt_user_id {
                let exit_message_loop = false;
                while !exit_message_loop {
                    let message = user_r_handle.get_message().await;
                    let games = self.games.read().await;
                    let game = games.get(&game_id).unwrap().read().await;
                    match message {
                        Input::ConfigureAndStart(csi)=>{
                            println!("Configure And Start Game Message");
                            let cwas = game.choose_winnings_and_start(user_id.clone(),csi.winnings.clone()).await;
                            println!("Game Started Internally {}",cwas);
                            if cwas {
                                game.send_messages_to_all(vec![Output::game_started(csi.winnings.clone())]).await;
                            }
                        },
                        Input::DrawNumber(dni)=>{
                            let draw_result = game.draw(user_id.clone(),dni.draw.clone()).await;
                            println!("Draw result {:?}",draw_result);
                            if let Some(num) = draw_result {
                                game.send_messages_to_all(vec![Output::new_number(num)]).await;
                            }
                        },
                        Input::ClaimNumber(cni)=>{
                            let result = game.claim_number(user_id.clone(),cni.number).await;
                            if result {
                                game.send_messages_to_user(user_id.clone(),vec![Output::claim_number_success(cni.number)]).await
                            } else {
                                game.send_messages_to_user(user_id.clone(),vec![Output::claim_number_failure(cni.number)]).await
                            }
                        },
                        Input::SendMessage(smi)=>{
                            game.send_messages_to_all(vec![Output::new_message(user_name.clone(),smi.message)]).await
                        },
                        Input::ClaimWin(cwi)=>{
                            let result = game.verify_and_mark_win(user_id.clone(),cwi.win_name).await;
                            if let Some(winner) = result {
                                game.send_messages_to_all(vec![Output::new_winner(winner.win_name,winner.user,winner.ticket)]).await
                            }
                        },
                        _=>{}
                    }
                };
            }
        }
    }
}