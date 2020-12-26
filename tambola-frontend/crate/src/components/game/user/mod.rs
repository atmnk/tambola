pub mod player;
pub mod host;
pub mod started;
use yew::prelude::*;
use yewtil::store::{ReadOnly,StoreWrapper,Bridgeable};
use agents::store::{TambolaStore, StoreInput};
use tambola_lib::game::{Winning, User, Ticket, PositionedNumber};
use yewtil::NeqAssign;
use yew::services::ConsoleService;
use components::game::{ValuedButton, DumbTicket};
use yew::agent::Dispatcher;
use agents::ws_api::{WSApi, Command};
use tambola_lib::game::proto::{Input, AnnouncementOutput};
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};
use yew_styles::layouts::item::AlignSelf;
use yew_styles::button::Button;
use yew_styles::styles::{Style, Size, Palette};
use components::game::user::player::Player;
use components::game::user::host::Host;

pub struct ClaimWinPanel{
    active_winnings:Vec<Winning>,
    link:ComponentLink<Self>,
    ws_api:Dispatcher<WSApi>,
    _store:Box<dyn Bridge<StoreWrapper<TambolaStore>>>
}
pub enum ClaimWinPanelMessage{
    StoreMessage(ReadOnly<TambolaStore>),
    ClaimWin(String)
}
impl Component for ClaimWinPanel{
    type Message = ClaimWinPanelMessage;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Self::Message::StoreMessage);
        let mut _store= TambolaStore::bridge(callback);
        _store.send(StoreInput::Spit);
        Self{
            link,
            active_winnings:vec![],
            ws_api:WSApi::dispatcher(),
            _store
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg{
            Self::Message::StoreMessage(store)=>{
                let new_winnings:Vec<Winning> =store.borrow().game_snapshot.as_ref().unwrap().winnings.clone();
                let active_winnings = new_winnings.iter().filter(|winning|{
                    winning.winner.is_none()
                }).map(|w|{w.clone()}).collect();
                if self.active_winnings.eq(&active_winnings) {
                    false
                } else {
                    self.active_winnings = active_winnings;
                    true
                }
            },
            Self::Message::ClaimWin(win)=>{
                self.ws_api.send(Command::SendData(Input::claim_win(win)));
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let buttons:Vec<Html> = self.active_winnings.iter().map(|winning|{
            html!{
                <ValuedButton onclick=self.link.callback(|value|{Self::Message::ClaimWin(value)}) value=winning.name.clone() class="">{&winning.name}</ValuedButton>
            }
        }).collect();
        html!{
            <div>{buttons}</div>
        }
    }
}

pub enum MessagePanelMessage{
    StoreMessage(ReadOnly<TambolaStore>),
}
pub struct MessagePanel{
    messages:Vec<AnnouncementOutput>,
    _store:Box<dyn Bridge<StoreWrapper<TambolaStore>>>,
}
impl Component for MessagePanel{
    type Message = MessagePanelMessage;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(MessagePanelMessage::StoreMessage);
        let mut _store = TambolaStore::bridge(callback);
        _store.send(StoreInput::Spit);
        Self{
            messages:vec![],
            _store
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg{
            Self::Message::StoreMessage(t_msg)=>{
                let sm = t_msg.borrow();
                if self.messages != sm.announcements{
                    self.messages = sm.announcements.clone();
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
        let inner:Vec<Html> = self.messages.iter().rev().map(|anc| {
            let prop = anc.clone();
            html!{
                <Announcemnet announcement = prop/>
            }
        }).collect();
        html!{
            <div>{inner}</div>
        }
    }
}

pub struct Announcemnet{
    props:AnnouncemnetProps
}
#[derive(Clone,Properties)]
pub struct AnnouncemnetProps{
    announcement:AnnouncementOutput
}
impl Component for Announcemnet{
    type Message = ();
    type Properties = AnnouncemnetProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self{
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let inner = match &self.props.announcement {
            AnnouncementOutput::GameStarted(gsa)=>{
                html!{
                    <div>{"Game Started"}</div>
                }
            },
            AnnouncementOutput::NewMessage(nmo)=>{
                html!{
                    <div>{&nmo.user}{":"}{&nmo.message}</div>
                }
            },
            AnnouncementOutput::NewNumber(nno)=>{
                html !{
                    <div>{"New Number is "}{&nno.number}</div>
                }
            },
            AnnouncementOutput::NewUserJoined(nujo)=>{
                html !{
                    <div>{&nujo.name}{" Joined"}</div>
                }
            },
            AnnouncementOutput::NewWinner(nwo)=>{

                html !{
                    <div>
                        <div>{&nwo.user_name}{" Won "}{&nwo.win_name}</div>
                        <div class="winner-ticket"><DumbTicket ticket=nwo.ticket.clone() enabled=false/></div>
                    </div>

                }
            }
        };
        html!{
            <div>{inner}</div>
        }
    }
}
pub struct UserScreen{
    props:UserScreenProps
}
#[derive(Clone,Properties,PartialEq)]
pub struct UserScreenProps{
    pub is_host:bool,
    pub user:User,
}
impl Component for UserScreen{
    type Message = ();
    type Properties = UserScreenProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self{
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
        let component = if self.props.is_host {
            html !{
                <Host user=self.props.user.clone()/>
            }
        } else {
            html!{
                <Player user=self.props.user.clone()/>
            }
        };
        html! {
            <Container direction=Direction::Row wrap=Wrap::Wrap>
                <Item layouts=vec!(ItemLayout::ItXs(9))>
                    {component}
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(3))>
                    <MessagePanel/>
                </Item>
            </Container>
        }
    }
}

pub struct UserTicket{
    props:UserTicketProps,
    link:ComponentLink<Self>,
    ws_api:Dispatcher<WSApi>,

}
#[derive(Clone,Properties,PartialEq)]
pub struct UserTicketProps{
    ticket:Ticket
}
pub enum UserTicketMessage{
    Clicked(u8)
}
impl Component for UserTicket{
    type Message = UserTicketMessage;
    type Properties = UserTicketProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self{
            ws_api:WSApi::dispatcher(),
            props,
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Clicked(num)=>{
                self.ws_api.send(Command::SendData(Input::claim_number(num)));
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <div class="user-ticket-area"><DumbTicket ticket=self.props.ticket.clone() enabled=true on_claim=self.link.callback(|num| Self::Message::Clicked(num))/></div>
        }
    }
}
