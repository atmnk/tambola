use yew::prelude::*;


use crate::components::game::Game;
use yew::services::{StorageService};
use yew::services::storage::Area;
use yew::format::Json;
use tambola_lib::game::User;

use agents::ws_api::{WSApi, Command};
use tambola_lib::game::proto::Input;
use uuid::Uuid;

pub struct GamePage{
    props:GamePageProps,
}
#[derive(Clone,Properties)]
pub struct GamePageProps{
    pub id:String
}
impl Component for GamePage {
    type Message = ();
    type Properties = GamePageProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        let mut ws_api = WSApi::dispatcher();
        if let Some(user) = StorageService::new(Area::Local)
            .ok()
            .and_then(|storage| {
                storage
                    .restore::<Json<anyhow::Result<User>>>("user")
                    .0
                    .ok()
            }) {
            ws_api.send(Command::SendData(Input::reconnect(user.id.clone(),Uuid::parse_str(props.id.as_str()).unwrap())));
        }
        GamePage {
            props,
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
            <Game id = self.props.id.clone()/>
            </div>
        }
    }
}
