use yew::prelude::*;
use yewtil::store::{StoreWrapper, Bridgeable, ReadOnly};
use agents::store::{TambolaStore, StoreInput};
pub enum PlayerMessage{
    StoreMessage(ReadOnly<TambolaStore>)
}
pub struct Player{
    name:String,
    store:Box<dyn Bridge<StoreWrapper<TambolaStore>>>,
}
impl Component for Player{
    type Message = PlayerMessage;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(PlayerMessage::StoreMessage);
        let mut store= TambolaStore::bridge(callback);
        store.send(StoreInput::Spit);
        Self{
            name:"Player".to_string(),
            store
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Self::Message::StoreMessage(rsm)=>{
                let sm = rsm.borrow();
                if sm.user.is_some(){
                    let user = sm.user.as_ref().unwrap();
                    self.name = user.name.clone();
                    true
                } else {
                    false
                }
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html !{
            <div>{format!("Welcome {}",self.name)}</div>
        }
    }
}