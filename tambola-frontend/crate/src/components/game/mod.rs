pub mod user;
use yew::prelude::*;
use yewtil::store::{Bridgeable, ReadOnly, StoreWrapper};
use agents::store::{TambolaStore};
use tambola_lib::game::{UserType, Ticket, PositionedNumber, User};


use yew::agent::Dispatcher;
use agents::ws_api::{WSApi, Command};
use tambola_lib::game::proto::{Input};
use yewtil::{NeqAssign};

use components::game::user::UserScreen;
use components::NameConnect;

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

pub enum GameMessage{
    StoreMessage(ReadOnly<TambolaStore>),
    ConnectMe(String),
}
pub struct Game{
    is_connected:bool,
    is_host:bool,
    user:Option<User>,
    ws_api:Dispatcher<WSApi>,
    props:GameProps,
    link:ComponentLink<Self>,
    _store:Box<dyn Bridge<StoreWrapper<TambolaStore>>>,
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
        let _store=TambolaStore::bridge(callback);
        Self{
            is_connected:false,
            is_host:false,
            ws_api:WSApi::dispatcher(),
            props,
            link,
            _store,
            user:Option::None
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg{
            Self::Message::StoreMessage(t_msg)=>{
                let mut update = false;
                let sm = t_msg.borrow();
                if sm.user.is_some(){
                    update = self.user.neq_assign(sm.user.clone());
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
                <UserScreen is_host = self.is_host user=self.user.clone().unwrap()/>
            }
        }

    }
}


pub struct DumbTicket{
    props:DumbTicketProps,
    link:ComponentLink<Self>
}
#[derive(Clone,Properties,PartialEq)]
pub struct DumbTicketProps{
    ticket:Ticket,
    enabled:bool,
    #[prop_or_else(default_callback)]
    on_claim:Callback<u8>,
}
pub enum DumbTicketMessage{
    Claimed(u8)
}
impl Component for DumbTicket{
    type Message = DumbTicketMessage;
    type Properties = DumbTicketProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self{
            props,
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Claimed(num)=>{
                if self.props.enabled{
                    self.props.on_claim.emit(num)
                }
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let cell_numbers=(0..27 as u8).collect::<Vec<u8>>();
        let cells :Vec<Html>= cell_numbers.iter().map(|index|{
            let row = index / 9;
            let column = index % 9;
            let pn = self.props.ticket.numbers.iter().find(|pn| {
                pn.row == row && pn.column == column
            }).map(|pn| {pn.clone()});
            let on_claim = pn.clone().map(|_pn| {self.link.callback(|num| {Self::Message::Claimed(num)})});
            html! {
                <TicketCell row=row column=column pn=pn on_claim=on_claim/>
            }
        }).collect();
        html !{
            <div class="ticket">
                {cells}
            </div>
        }
    }
}
pub fn default_callback()->Callback<u8>{
    Callback::noop()
}
pub struct TicketCell{
    props:TicketCellProps,
    link:ComponentLink<Self>
}
#[derive(Clone,Properties,PartialEq)]
pub struct TicketCellProps{
    row:u8,
    column:u8,
    pn:Option<PositionedNumber>,
    on_claim:Option<Callback<u8>>
}
pub enum TicketCellMessage{
    Clicked
}
impl Component for TicketCell{
    type Message = TicketCellMessage;
    type Properties = TicketCellProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self{
            link,
            props
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Clicked=>{
                self.props.on_claim.as_ref().map(|cb| {
                    cb.emit(self.props.pn.clone().unwrap().number)
                });
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        if self.props.pn.is_some() {
            let claimed = self.props.pn.as_ref().map(|pn| {if pn.claimed {"cell claimed"} else {"cell numbered"}}).unwrap_or("cell");
            let text = self.props.pn.as_ref().map(|ip|{ip.number.to_string()}).unwrap_or("".to_string());
            let inner:Html = html! {
                <ResponsiveText text=text/>
            };
            if self.props.on_claim.is_some() {
                html! {
                    <button class=claimed onclick=self.link.callback(|_|Self::Message::Clicked)>{inner}</button>
                }
            } else {
                html! {
                    <button class=claimed>{inner}</button>
                }
            }
        } else {
            html! {
                <button class="cell empty"></button>
            }
        }

    }
}

