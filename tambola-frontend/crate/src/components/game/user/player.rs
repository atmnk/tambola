use yew::prelude::*;
use yewtil::store::{StoreWrapper, Bridgeable, ReadOnly};
use agents::store::{TambolaStore, StoreInput};
use tambola_lib::game::User;
use yewtil::NeqAssign;
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};
use yew_styles::layouts::item::AlignSelf;
use yew_styles::button::Button;
use yew_styles::styles::{Style, Size, Palette};
use components::game::user::{ClaimWinPanel, UserTicket};

pub struct DoneNumbers{
    done:Vec<u8>,
    _store:Box<dyn Bridge<StoreWrapper<TambolaStore>>>,
}
pub enum DoneNumberMessage{
    StoreMessage(ReadOnly<TambolaStore>),
}
impl Component for DoneNumbers{
    type Message = DoneNumberMessage;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(DoneNumberMessage::StoreMessage);
        let mut _store= TambolaStore::bridge(callback);
        _store.send(StoreInput::Spit);
        Self{
            done:vec![],
            _store
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Self::Message::StoreMessage(str)=>{
                let new_numbers = str.borrow().game_snapshot.as_ref().unwrap().done_numbers.clone();
                if self.done == new_numbers {
                    false
                } else {
                    self.done =new_numbers;
                    true
                }
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let numbers :Vec<Html> = self.done.clone().iter().map(|num|{
            html! {
                <div>{num}</div>
            }
        }).collect();
        html!{
            <div>
                {numbers}
            </div>
        }
    }
}

pub enum PlayerMessage{
    StoreMessage(ReadOnly<TambolaStore>)
}
pub struct Player{
    name:String,
    props:PlayerProps,
    _store:Box<dyn Bridge<StoreWrapper<TambolaStore>>>,
}
#[derive(Clone,PartialEq,Properties)]
pub struct PlayerProps{
    pub user:User
}
impl Component for Player{
    type Message = PlayerMessage;
    type Properties = PlayerProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(PlayerMessage::StoreMessage);
        let mut _store= TambolaStore::bridge(callback);
        _store.send(StoreInput::Spit);
        Self{
            props,
            name:"Player".to_string(),
            _store
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

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html !{
            <Container direction=Direction::Column wrap=Wrap::Wrap>
                <Item layouts=vec!(ItemLayout::ItXs(12)) align_self=AlignSelf::Center>
                    <DoneNumbers/>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(12)) align_self=AlignSelf::Center>
                    <UserTicket ticket=self.props.user.ticket.clone()/>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(12)) align_self=AlignSelf::Stretch>
                    <ClaimWinPanel/>
                </Item>
            </Container>
        }
    }
}
