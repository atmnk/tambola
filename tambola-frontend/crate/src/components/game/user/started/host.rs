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
use components::game::user::{ClaimWinPanel, UserTicket};
use components::game::{ResponsiveText, ValuedButton};

pub enum NumberDrawerMessage{
    DrawNumber(u8),
    DrawRandom,
    StoreMessage(ReadOnly<TambolaStore>)
}
pub struct NumberDrawer{
    link:ComponentLink<Self>,
    ws_api:Dispatcher<WSApi>,
    gs:GameSnapshot,
    _store:Box<dyn Bridge<StoreWrapper<TambolaStore>>>,
}
impl Component for NumberDrawer{
    type Message = NumberDrawerMessage;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(NumberDrawerMessage::StoreMessage);
        let mut _store= TambolaStore::bridge(callback);
        _store.send(StoreInput::Spit);
        Self {
            link,
            gs:GameSnapshot::blank(),
            ws_api:WSApi::dispatcher(),
            _store
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {

        match msg {
            Self::Message::DrawNumber(num)=>{
                ConsoleService::log(format!("Drawing {}",num).as_str());
                self.ws_api.send(Command::SendData(Input::draw(Draw::Specific(num))));
                false
            },
            Self::Message::DrawRandom=>{
                self.ws_api.send(Command::SendData(Input::draw(Draw::Random)));
                false
            },
            Self::Message::StoreMessage(store)=>{
                self.gs.neq_assign(store.borrow().game_snapshot.as_ref().unwrap().clone())
            }
        }

    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let numbers = (1..91 as u8).collect::<Vec<u8>>();
        let inner :Vec<Html> = numbers.iter().map(|num|{
            let class = if self.gs.done_numbers.contains(num){
                "cell done"
            } else {
                "cell"
            };
            html!{
                <ValuedButton value={num.to_string()} class={class} onclick=self.link.callback(|value:String|NumberDrawerMessage::DrawNumber(value.parse::<u8>().unwrap()))><ResponsiveText text={num.to_string()}/></ValuedButton>

           }
        }).collect();
        html !{
            <div class="component">
                <div class={"drawer-grid"}>{inner}</div>
                <button onclick=self.link.callback(|_|NumberDrawerMessage::DrawRandom)>{"Draw Random"}</button>
            </div>
        }
    }
}

pub struct GamePanel{
    props:GamePanelProps
}
#[derive(Clone,Properties,PartialEq)]
pub struct GamePanelProps{
    pub user:User
}
impl Component for GamePanel{
    type Message = ();
    type Properties = GamePanelProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {

        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
             <Container direction=Direction::Column wrap=Wrap::Wrap>
                <Item layouts=vec!(ItemLayout::ItXs(12)) align_self=AlignSelf::Center>
                    <NumberDrawer/>
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