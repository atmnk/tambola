use yew::prelude::*;
use yewtil::store::{Bridgeable, ReadOnly, StoreWrapper, Store};
use agents::{CounterStore, Request};
use yew::agent::{AgentLink, Dispatcher};
use agents::ws_api::{WSApi, Command};

pub struct CounterDisplay{
    value:usize,
    link:ComponentLink<Self>,
    counter_store:Box<dyn Bridge<StoreWrapper<CounterStore>>>
}
pub enum CounterDisplayMessage{
    SetTo(usize)
}
pub struct ActionPanel{
    value:usize,
    link:ComponentLink<Self>,
    wsapi:Dispatcher<WSApi>,
    counter_store:Box<dyn Bridge<StoreWrapper<CounterStore>>>
}
pub enum ActionPanelMessage {
    Increment,
    Decrement,
    Double,
    CounterStoreMessage(ReadOnly<CounterStore>)
}
impl Component for ActionPanel{
    type Message = ActionPanelMessage;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(ActionPanelMessage::CounterStoreMessage);
        Self{
            wsapi:WSApi::dispatcher(),
            value:CounterStore::new().value,
            link,
            counter_store:CounterStore::bridge(callback)
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            ActionPanelMessage::Increment=>{
                self.wsapi.send(Command::Connect);
                self.counter_store.send(Request::Increment);
            },
            ActionPanelMessage::Decrement=>{
                self.counter_store.send(Request::Decrement);
            },
            ActionPanelMessage::Double=>{
                self.counter_store.send(Request::SetTo(self.value * 2))
            }
            ActionPanelMessage::CounterStoreMessage(changed)=>{
                self.value = changed.borrow().value
            }
        }
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html!{
            <div>
                <button onclick = self.link.callback(|_|Self::Message::Increment)>{"Increament"}</button>
                <button onclick = self.link.callback(|_|Self::Message::Decrement)>{"Decrement"}</button>
                <button onclick = self.link.callback(|_|Self::Message::Double)>{"Double"}</button>
            </div>
        }
    }
}
impl Component for CounterDisplay{
    type Message = CounterDisplayMessage;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|msg:ReadOnly<CounterStore>|{CounterDisplayMessage::SetTo(msg.borrow().value)});
        Self {
            value:CounterStore::new().value,
            link,
            counter_store:CounterStore::bridge(callback)

        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            CounterDisplayMessage::SetTo(val)=>{
                self.value = val;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>{self.value}</div>
        }
    }
}
pub struct App;
impl Component for App{
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <CounterDisplay/>
                <CounterDisplay/>
                <ActionPanel/>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}