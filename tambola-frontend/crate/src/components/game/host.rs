use yew::prelude::*;
use yew::agent::Dispatcher;
use agents::ws_api::{WSApi, Command};
use tambola_lib::game::proto::Input;
use tambola_lib::game::{Winning, WinningVerifier, Draw, GameSnapshot};
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
use components::game::{UserTicket, ClaimWinPanel, ResponsiveText, ValuedButton};
use yew::services::DialogService;
use yewtil::NeqAssign;

pub enum HostMessage{
    StoreMessage(ReadOnly<TambolaStore>),
}
pub struct Host{
    store:Box<dyn Bridge<StoreWrapper<TambolaStore>>>,
    game_started:bool
}
impl Component for Host{
    type Message = HostMessage;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(HostMessage::StoreMessage);
        let mut store= TambolaStore::bridge(callback);
        store.send(StoreInput::Spit);
        Self{
            store,
            game_started:false
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

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        if self.game_started {
            html !{
                <GamePanel/>
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
pub enum NumberDrawerMessage{
    DrawNumber(u8),
    DrawRandom,
    StoreMessage(ReadOnly<TambolaStore>)
}
pub struct NumberDrawer{
    link:ComponentLink<Self>,
    ws_api:Dispatcher<WSApi>,
    gs:GameSnapshot,
    store:Box<dyn Bridge<StoreWrapper<TambolaStore>>>
}
impl Component for NumberDrawer{
    type Message = NumberDrawerMessage;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(NumberDrawerMessage::StoreMessage);
        let mut store= TambolaStore::bridge(callback);
        store.send(StoreInput::Spit);
        Self {
            link,
            gs:GameSnapshot::blank(),
            store,
            ws_api:WSApi::dispatcher()
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {

        match msg {
            Self::Message::DrawNumber(num)=>{
                DialogService::alert(format!("Drawing {}",num).as_str());
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
pub struct GamePanel;
impl Component for GamePanel{
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
             <Container direction=Direction::Column wrap=Wrap::Wrap>
                <Item layouts=vec!(ItemLayout::ItXs(12)) align_self=AlignSelf::Center>
                    <NumberDrawer/>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(12)) align_self=AlignSelf::Center>
                    <UserTicket/>
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(12)) align_self=AlignSelf::Stretch>
                    <ClaimWinPanel/>
                </Item>
            </Container>
        }
    }
}