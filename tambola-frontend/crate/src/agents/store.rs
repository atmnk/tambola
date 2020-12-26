use yewtil::store::{Store, StoreWrapper};
use yew::agent::AgentLink;
use yew::services::{DialogService, ConsoleService};
use yew::{Bridge, Bridged};
use agents::ws_api::{WSApi};
use tambola_lib::game::{User, GameSnapshot, Winning, PositionedNumber};
use tambola_lib::game::proto::AnnouncementOutput;

pub enum Action{
    SetAnnouncements(Vec<AnnouncementOutput>),
    SetConnected(bool),
    SetUser(User),
    SetSnapshot(GameSnapshot),
    AddAnnouncement(AnnouncementOutput),
    ClaimSuccess(Vec<PositionedNumber>),
    Spit,
}
pub enum StoreInput{
    Connected,
    NewGameHosted(String,User),
    Reconnected(User,GameSnapshot,Vec<AnnouncementOutput>),
    NewAnnouncement(AnnouncementOutput),
    ConnectedToGame(User,GameSnapshot,Vec<AnnouncementOutput>),
    ClaimSuccess(u8),
    Spit
}
pub struct TambolaStore{
    pub ws_connected:bool,
    pub user:Option<User>,
    pub game:Option<String>,
    pub game_snapshot:Option<GameSnapshot>,
    pub announcements:Vec<AnnouncementOutput>,
}
impl Store for TambolaStore{

    type Input = StoreInput;
    type Action = Action;


    fn new() -> Self {
        Self{
            ws_connected:false,
            user:None,
            game:None,
            game_snapshot:None,
            announcements:vec![]
        }
    }

    fn handle_input(&self, link: AgentLink<StoreWrapper<Self>>, msg: Self::Input) {
        match msg {
            StoreInput::Connected=>{
                link.send_message(Action::SetConnected(true));
            },
            StoreInput::NewGameHosted(game_id,user)=>{
                link.send_message(Action::SetUser(user));
            },
            StoreInput::Reconnected(user,gss,anc)=>{
                link.send_message(Action::SetSnapshot(gss));
                link.send_message(Action::SetAnnouncements(anc));
                link.send_message(Action::SetUser(user));
            },
            StoreInput::NewAnnouncement(an)=>{
                link.send_message(Action::AddAnnouncement(an))
            }
            StoreInput::Spit=>{
                link.send_message(Action::Spit)
            },
            StoreInput::ConnectedToGame(user,gss,anc)=>{
                link.send_message(Action::SetSnapshot(gss));
                link.send_message(Action::SetAnnouncements(anc));
                link.send_message(Action::SetUser(user));
            },
            StoreInput::ClaimSuccess(num)=>{
                let new_numbers = self.user.as_ref().unwrap().ticket.numbers.iter().map(|pn|{ if &pn.number == &num {
                    PositionedNumber{
                        number:pn.number.clone(),
                        row:pn.row.clone(),
                        column:pn.column.clone(),
                        claimed:true
                    }
                } else {
                    pn.clone()
                }}).collect::<Vec<PositionedNumber>>();
                link.send_message(Action::ClaimSuccess(new_numbers));
            }
        }
    }

    fn reduce(&mut self, msg: Self::Action) {
        match msg {
            Action::SetConnected(val)=>{
                self.ws_connected = val;
            },
            Action::SetUser(user)=>{
                self.user = Some(user);
            },
            Action::SetSnapshot(sn)=>{
                self.game_snapshot=Some(sn)
            },
            Action::Spit=>{

            },
            Action::SetAnnouncements(anc)=>{
                self.announcements = anc;
            }
            Action::AddAnnouncement(an)=>{
                self.announcements.push(an.clone());
                match an {
                    AnnouncementOutput::GameStarted(gsa)=>{
                        self.game_snapshot = Some(GameSnapshot{
                            started:true,
                            done_numbers:vec![],
                            winnings:gsa.winnings
                        })
                    },
                    AnnouncementOutput::NewNumber(nna)=>{
                        if let Some(gs) = &mut self.game_snapshot{
                            gs.done_numbers.push(nna.number)
                        }
                    },
                    AnnouncementOutput::NewWinner(nwa)=>{
                        let new_winnings = self.game_snapshot.as_ref().unwrap().winnings.iter().map(|winning|{
                            if winning.name == nwa.win_name {
                                Winning{
                                    name:nwa.win_name.clone(),
                                    winner:Option::Some(nwa.user_name.clone()),
                                    verify_by:winning.verify_by.clone()
                                }
                            } else {
                                winning.clone()
                            }
                        }).collect();
                        self.game_snapshot = Some(GameSnapshot{
                            started:true,
                            done_numbers:self.game_snapshot.as_ref().unwrap().done_numbers.clone(),
                            winnings:new_winnings
                        })
                    }
                    _=>{}

                }

            },
            Action::ClaimSuccess(numbers)=>{
                self.user = self.user.as_ref().map(|user|{
                    let mut new_user = user.clone();
                    new_user.ticket.numbers = numbers;
                    new_user
                });
            }

        }
    }
}