use yewtil::store::{Store, StoreWrapper};
use yew::agent::AgentLink;
pub mod ws_api;
pub mod store;
pub enum Request{
    Increment,
    SetTo(usize),
    Reset,
    Decrement
}
pub enum Action{
    SetValue(usize)
}
pub struct CounterStore{
    pub value:usize
}
impl Store for CounterStore{
    type Input = Request;
    type Action = Action;

    fn new() -> Self {
        CounterStore{
            value:3,
        }
    }

    fn handle_input(&self, link: AgentLink<StoreWrapper<Self>>, msg: Self::Input) {
        match msg {
            Request::Increment=>{
                link.send_message(Action::SetValue(self.value + 1))
            },
            Request::Decrement=>{
                if self.value > 0 {
                    link.send_message(Action::SetValue(self.value - 1))
                }
            },
            Request::SetTo(val)=>{
                link.send_message(Action::SetValue(val))
            },
            Request::Reset=>{
                link.send_message(Action::SetValue(0))
            }
        }
    }

    fn reduce(&mut self, msg: Self::Action) {
        match msg {
            Action::SetValue(val)=>{
                self.value = val
            }
        }
    }
}