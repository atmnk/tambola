use tokio::sync::RwLock;
use std::collections::HashMap;
use uuid::Uuid;
use tambola_lib::game::{GameSnapshot, Winner, Winning, Draw, UserType, Ticket, User};
use crate::server::UserWSSHandle;
use tambola_lib::game::proto::Output;

pub mod user;

pub struct GameInstance {
    pub id:Uuid,
    pub host:Uuid,
    pub users:RwLock<HashMap<Uuid,RwLock<User>>>,
    pub snapshot:RwLock<GameSnapshot>,
    pub user_handles : RwLock<HashMap<Uuid, RwLock<UserWSSHandle>>>
}

impl GameInstance {
    pub async fn send_messages_properly(&self,handle:UserWSSHandle,messages:Vec<Output>)->UserWSSHandle{
        let users = self.user_handles.read().await;
        for message in messages  {
            match message {
                Output::Announcement(ao)=>{
                    for (_,user_lk) in users.iter() {
                        let user = user_lk.read().await;
                        user.send(Output::Announcement(ao.clone()))
                    }
                },
                _=> {
                    handle.send(message)
                }
            }
        }
        handle
    }
    pub async fn send_messages_to_user(&self,user_id:Uuid,messages:Vec<Output>){
        let users = self.user_handles.read().await;
        if let Some(rw_user) = users.get(&user_id) {
            let user = rw_user.read().await;
            for message in messages  {
                user.send(message)
            }
        }
    }
    pub async fn send_messages_to_all(&self,messages:Vec<Output>){
        let users = self.user_handles.read().await;
        for message in messages  {
            match message {
                Output::Announcement(ao)=>{
                    for (_,user_lk) in users.iter() {
                        let user = user_lk.read().await;
                        user.send(Output::Announcement(ao.clone()))
                    }
                },
                _=> {
                }
            }
        }
    }
    pub async fn insert_handle(&self,user_id:Uuid,handle:UserWSSHandle){
        let mut handles = self.user_handles.write().await;
        handles.insert(user_id,RwLock::new(handle));
    }
    pub async fn reconnect(&self,user_id:Uuid)->Option<User>{
        let users = self.users.read().await;
        if let Some(rw_user) = users.get(&user_id){
            let user = rw_user.read().await;
            Some(user.clone())
        } else {
            None
        }
    }
    pub async fn choose_winnings_and_start(&self,user:Uuid,winnings:Vec<Winning>)->bool{
        if(self.id.clone().eq(&user)) {
            let mut ss = self.snapshot.write().await;
            ss.winnings = winnings.iter().map(|w| {
                let mut winning = w.clone();
                winning.winner = Option::None;
                winning
            }).collect();
            ss.started  = true ;
            true
        } else {
            false
        }
    }
    pub async fn is_game_started(&self)->bool{
        let ss = self.snapshot.read().await;
        ss.started.clone()
    }
    pub async fn claim_number(&self,user:Uuid,number:u8)->bool{
        if self.is_game_started().await {
            let users = self.users.read().await;
            let opt_user = users.get(&user.clone());
            if let Some(rw_user) = opt_user {
                let mut user = rw_user.write().await;
                user.ticket.claim_number(number)
            } else {
                false
            }
        } else {
            false
        }

    }
    pub async fn draw(&self,user:Uuid,draw:Draw)->Option<u8>{
        if user.clone().eq(&self.host){
            let mut ss= self.snapshot.write().await;
            ss.draw_number(draw)
        } else {
            Option::None
        }
    }
    pub async fn join(&self,name:String)->User{
        let user_id = Uuid::new_v4();
        let user = User{
            id:user_id.clone(),
            name:name.clone(),
            user_type:UserType::NonHost,
            ticket:Ticket::default()
        };
        let mut users = self.users.write().await;
        users.insert(user.id.clone(),RwLock::new(user.clone()));
        user
    }
    pub async fn verify_and_mark_win(&self,user:Uuid,win_name:String)->Option<Winner>{
        if self.is_game_started().await {
            let opt_win= {
                let game_snapshot = self.snapshot.read().await;
                game_snapshot.winnings.iter().find(|winning| winning.name.clone() == win_name).map(|w| w.clone())
            };
            let otp_ticket = {
                let users = self.users.read().await;
                let opt_user = users.get(&user.clone());
                if let Some(rw_user) = opt_user {
                    let user = rw_user.read().await;
                    Option::Some((user.name.clone() ,user.ticket.clone()))
                } else {
                    Option::None
                }
            };
            let game_snapshot = {
                let gn = self.snapshot.read().await;
                gn.clone()
            };
            if let Some((user_name,ticket)) = otp_ticket {
                if let Some(win) = opt_win {
                    if win.verify_by.verify(&game_snapshot,user.clone(),&ticket) {
                        let mut gs = self.snapshot.write().await;
                        gs.mark_winner(win_name.clone(),user.clone());
                        Option::Some(Winner{
                            user:user_name.clone(),
                            win_name:win_name.clone(),
                            ticket:ticket.clone()
                        })
                    } else {
                        Option::None
                    }
                } else {
                    Option::None
                }
            } else {
                Option::None
            }
        } else {
            Option::None
        }

    }
    pub async fn mark_number_done(&self,number:u8)->bool{
        if self.is_game_started().await {
            let mut snapshot = self.snapshot.write().await;
            snapshot.mark_number_done(number);
            true
        } else {
            false
        }

    }
}