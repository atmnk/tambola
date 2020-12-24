use yew::prelude::*;
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};
use yew_styles::layouts::item::AlignSelf;
use yew_styles::button::Button;
use yew_styles::styles::{Style, Size, Palette};
use yew::services::DialogService;
use agents::ws_api::{WSApi, Command};
use yew::agent::Dispatcher;
use tambola_lib::game::proto::Input;
use components::NameConnect;

pub enum NewGameMessage{
    StartNewGame(String)
}
pub struct NewGame{
    link:ComponentLink<Self>,
    ws_api:Dispatcher<WSApi>
}
impl Component for NewGame{
    type Message = NewGameMessage;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self{
            link,
            ws_api:WSApi::dispatcher(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            NewGameMessage::StartNewGame(name)=>{
                self.ws_api.send(Command::SendData(Input::new_game(name.clone())));
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {

        false
    }

    fn view(&self) -> Html {
        html! {
            <NameConnect label={"Start New Game"} onsubmit=self.link.callback(NewGameMessage::StartNewGame)/>
        }
    }
}