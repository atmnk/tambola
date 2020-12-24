use yewtil::store::{Store, StoreWrapper};
use yew::agent::AgentLink;
use yew::services::DialogService;
use yew::{Bridge, Bridged};
use agents::ws_api::{WSApi};
use tambola_lib::game::{User, GameSnapshot, Winning};
use tambola_lib::game::proto::AnnouncementOutput;

pub enum Action{
    SetConnected(bool),
    SetUser(User),
    SetSnapshot(GameSnapshot),
    AddAnnouncement(AnnouncementOutput),
    Spit,
}
pub enum StoreInput{
    Connected,
    NewGameHosted(String,User),
    Reconnected(User,GameSnapshot),
    NewAnnouncement(AnnouncementOutput),
    ConnectedToGame(User,GameSnapshot),
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
            StoreInput::Reconnected(user,gss)=>{
                link.send_message(Action::SetSnapshot(gss));
                link.send_message(Action::SetUser(user));
            },
            StoreInput::NewAnnouncement(an)=>{
                link.send_message(Action::AddAnnouncement(an))
            }
            StoreInput::Spit=>{
                link.send_message(Action::Spit)
            },
            StoreInput::ConnectedToGame(user,gss)=>{
                link.send_message(Action::SetSnapshot(gss));
                link.send_message(Action::SetUser(user));
            },
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
                    }
                    _=>{}

                }

            }

        }
    }
}