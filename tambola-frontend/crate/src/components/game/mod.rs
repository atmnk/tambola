use yew::prelude::*;
use yewtil::store::{Bridgeable, ReadOnly, StoreWrapper};
use agents::store::{TambolaStore, StoreInput};
use tambola_lib::game::UserType;
use components::game::host::Host;
use components::game::player::Player;
use yew::services::DialogService;
use components::NameConnect;
use yew::agent::Dispatcher;
use agents::ws_api::{WSApi, Command};
use tambola_lib::game::proto::{Input, AnnouncementOutput};
pub struct ResponsiveText{
    pub props:ResponsiveTextProps
}
#[derive(Properties,Clone)]
pub struct ResponsiveTextProps{
    pub text:String
}
impl Component for ResponsiveText{
    type Message = ();
    type Properties = ResponsiveTextProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        ResponsiveText{
            props
        }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        true
    }

    fn view(&self) -> Html {
        html!{
            <svg
            width="100%"
            height="100%"
            viewBox="0 0 500 75"
            preserveAspectRatio="xMinYMid meet"
            xmlns="http://www.w3.org/2000/svg"
          >
                <text
                  x="50%" y="50%" dominant-baseline="middle" text-anchor="middle"
                  font-size="320"
                  fill="currentColor"
                >{self.props.text.clone()}</text>
            </svg>
        }
    }
}
pub struct ValuedButton{
    props:ValuedButtonProps,
    link:ComponentLink<Self>
}
#[derive(Clone,Properties)]
pub struct ValuedButtonProps{
    pub onclick:Callback<String>,
    pub value:String,
    pub children:Children,
    pub class:String,
}
pub enum ValuedButtonMessage{
    Clicked
}
impl  Component for ValuedButton{
    type Message = ValuedButtonMessage;
    type Properties = ValuedButtonProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self{
            link,
            props
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Clicked=>{
                self.props.onclick.emit(self.props.value.clone());
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html!{
            <button onclick=self.link.callback(|_|ValuedButtonMessage::Clicked) class=&self.props.class>{self.props.children.clone()}</button>
        }
    }
}
pub mod host;
pub mod player;
pub enum GameMessage{
    StoreMessage(ReadOnly<TambolaStore>),
    ConnectMe(String),
}
pub struct Game{
    is_connected:bool,
    is_host:bool,
    ws_api:Dispatcher<WSApi>,
    props:GameProps,
    link:ComponentLink<Self>,
    store:Box<dyn Bridge<StoreWrapper<TambolaStore>>>,
}
pub enum MessagePanelMessage{
    StoreMessage(ReadOnly<TambolaStore>),
}
pub struct MessagePanel{
    messages:Vec<AnnouncementOutput>,
    store:Box<dyn Bridge<StoreWrapper<TambolaStore>>>,
}
impl Component for MessagePanel{
    type Message = MessagePanelMessage;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(MessagePanelMessage::StoreMessage);
        let mut store = TambolaStore::bridge(callback);
        store.send(StoreInput::Spit);
        Self{
            messages:vec![],
            store
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg{
            Self::Message::StoreMessage(t_msg)=>{
                let mut update = false;
                let sm = t_msg.borrow();
                self.messages = sm.announcements.clone();
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let inner:Vec<Html> = self.messages.iter().map(|anc| {
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
                    <div>{&nwo.user_name}{" Won "}{&nwo.win_name}</div>
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
#[derive(Clone,Properties)]
pub struct UserScreenProps{
    is_host:bool
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
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let component = if self.props.is_host {
            html !{
                <Host/>
            }
        } else {
            html!{
                <Player/>
            }
        };
        html! {
            <div>
                <MessagePanel/>
                {component}
            </div>
        }
    }
}
#[derive(Clone,Properties)]
pub struct GameProps{
    pub id:String
}
impl Component for Game{
    type Message = GameMessage;
    type Properties = GameProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(GameMessage::StoreMessage);
        Self{
            is_connected:false,
            is_host:false,
            ws_api:WSApi::dispatcher(),
            props,
            link,
            store:TambolaStore::bridge(callback)
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg{
            Self::Message::StoreMessage(t_msg)=>{
                let mut update = false;
                let sm = t_msg.borrow();
                if sm.user.is_some(){
                    if !self.is_connected {
                        self.is_connected = true;
                        update = true;
                    }
                    match sm.user.clone().unwrap().user_type {
                        UserType::Host=>{
                            if !self.is_host {
                                self.is_host = true;
                                update = true;
                            }
                        },
                        UserType::NonHost=>{
                            if self.is_host {
                                self.is_host = false;
                                update = true
                            }
                        }
                    }
                } else {
                    if self.is_connected {
                        self.is_connected = false;
                        update = true;
                    }
                }

                update
            },
            Self::Message::ConnectMe(name)=>{
                self.ws_api.send(Command::SendData(Input::connect_me_as(name,self.props.id.clone())));
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        if !self.is_connected {
            html!{
                <NameConnect label={"Connect Me"} onsubmit=self.link.callback(GameMessage::ConnectMe)/>
            }
        } else {
            html! {
                <UserScreen is_host = self.is_host/>
            }
        }

    }
}



pub struct UserTicket;
impl Component for UserTicket{
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
            <div>{"User Ticket"}</div>
        }
    }
}
pub struct ClaimWinPanel;
impl Component for ClaimWinPanel{
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
            <div>{"Claim Win Panel"}</div>
        }
    }
}

