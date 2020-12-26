use yew::prelude::*;
use yew::agent::{Bridged};
use agents::ws_api::{WSApi};


use yewtil::store::{Bridgeable, ReadOnly, StoreWrapper};
use agents::store::TambolaStore;

use yewtil::NeqAssign;

pub mod home;
pub mod game;
pub struct TambolaPage{
    is_connected:bool,
    props:TambolaPageProps,
    _store:Box<dyn Bridge<StoreWrapper<TambolaStore>>>,
    _ws_api:Box<dyn Bridge<WSApi>>,
}
pub enum PageMessage{
    StoreMessage(ReadOnly<TambolaStore>),
    None,
}
#[derive(Clone, Properties,PartialEq)]
pub struct TambolaPageProps{
    children:Children
}
impl Component for TambolaPage {
    type Message = PageMessage;
    type Properties = TambolaPageProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(PageMessage::StoreMessage);
        let store= TambolaStore::bridge(callback);
        TambolaPage{
            is_connected:false,
            props,
            _store:store,
            _ws_api:WSApi::bridge(link.callback(|_|{PageMessage::None}))
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            PageMessage::StoreMessage(msg)=>{
                let is_connected_from_store = msg.borrow().ws_connected;
                if is_connected_from_store && self.is_connected == false {
                    self.is_connected = true;
                    true
                } else {
                    false
                }
            },
            _=>{
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props.neq_assign(props)
    }
    fn view(&self) -> Html {
        let is_connected = if self.is_connected{
            "".to_string()
        } else {
            "not".to_string()
        };
        html! {
            <div class="page">
                <div>{ format!("You are {} Connected",is_connected)}</div>
                { self.props.children.clone() }
            </div>
        }
    }
}
