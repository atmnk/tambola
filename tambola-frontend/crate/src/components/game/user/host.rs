use yew::prelude::*;
use yew::agent::Dispatcher;
use agents::ws_api::{WSApi, Command};
use tambola_lib::game::proto::Input;
use tambola_lib::game::{Winning, WinningVerifier, Draw, GameSnapshot, Ticket, User};
use yewtil::store::{ReadOnly, StoreWrapper,Bridgeable};
use agents::store::{TambolaStore, StoreInput};
use web_sys::MouseEvent;
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};
use yew_styles::layouts::item::AlignSelf;
use yew_styles::button::Button;
use yew_styles::styles::{Style, Size, Palette};

use yew::services::{DialogService, ConsoleService};
use yewtil::NeqAssign;
use components::game::user::started::host::GamePanel;
use components::game::user::{UserTicket, ClaimWinPanel};
use components::game::{ResponsiveText, ValuedButton};

pub enum HostMessage{
    StoreMessage(ReadOnly<TambolaStore>),
}
pub struct Host{
    game_started:bool,
    props:HostProps,_store:Box<dyn Bridge<StoreWrapper<TambolaStore>>>,
}
#[derive(Clone,PartialEq,Properties)]
pub struct HostProps{
    pub user:User,
}
impl Component for Host{
    type Message = HostMessage;
    type Properties = HostProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {

        let callback = link.callback(HostMessage::StoreMessage);
        let mut _store= TambolaStore::bridge(callback);
        _store.send(StoreInput::Spit);
        Self{
            game_started:false,
            props,
            _store
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Self::Message::StoreMessage(rsm)=>{
                let sm = rsm.borrow();
                if sm.game_snapshot.is_some(){
                    let gs = sm.game_snapshot.as_ref().unwrap();
                    self.game_started=gs.started.clone();
                    true
                } else {
                    false
                }
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        if self.game_started {
            html !{
                <GamePanel user=self.props.user.clone()/>
            }
        } else {
            html !{
                <ConfigureAndStartGame/>
            }
        }

    }
}
pub struct ConfigureAndStartGame{
    ws_api:Dispatcher<WSApi>,
    link:ComponentLink<Self>
}
pub enum ConfigureAndStartGameMessage{
    StartGame,
}
impl Component for ConfigureAndStartGame{
    type Message = ConfigureAndStartGameMessage;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self{
            ws_api:WSApi::dispatcher(),
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Self::Message::StartGame=>{
                let winnings= vec![Winning{
                    name:"Love At First Sight".to_string(),
                    verify_by:WinningVerifier::LoveAtFirstSight,
                    winner:None
                }];
                self.ws_api.send(Command::SendData(Input::configure_and_start_game(winnings)));
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        true
    }

    fn view(&self) -> Html {
        html! {
            <button class = "start" onclick = self.link.callback(|_|ConfigureAndStartGameMessage::StartGame)>{"Start Game"}</button>
        }
    }
}