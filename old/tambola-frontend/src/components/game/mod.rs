use tambola_lib::game::PositionedNumber;
use yew::prelude::*;
use crate::components::ResponsiveText;
use yew::services::{DialogService, ConsoleService};

pub struct Cell{
    pub props:CellProps,
    link:ComponentLink<Self>
}
#[derive(Clone, Properties,PartialEq)]
pub struct CellProps{
    pub row:u8,
    pub column:u8,
    pub value:Option<PositionedNumber>,
    pub parent_callback:Callback<u8>,
    pub disabled:bool
}
pub struct Ticket{
    link: ComponentLink<Self>,
    pub props:TicketProps
}
#[derive(Clone, Properties,PartialEq)]
pub struct TicketProps{
    pub positioned_numbers:Vec<PositionedNumber>,
    pub disabled:bool,
}
pub enum TicketMessage{
    Clicked(u8)
}
pub enum CellMsg {
    Clicked,
}
impl Component for Cell {
    type Message = CellMsg;
    type Properties = CellProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Cell {
            props,
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            CellMsg::Clicked => {
                if !self.props.disabled {
                    self.props.parent_callback.emit(self.props.value.clone().unwrap().number);
                }
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let value = self.props.value.clone().map(|val| val.number.to_string()).unwrap_or("".to_string());
        let claimed = if let Some(pn) = self.props.value.clone() {
            if pn.claimed {
                " claimed"
            } else {
                " "
            }
        } else {
            " "
        };
        let onclick = if let Some(_) = self.props.value.clone(){
            true
        } else {
            false
        };
        if onclick {
            return html! {
                <button class = {format!("cell {}",claimed) } onclick=self.link.callback(|_| CellMsg::Clicked)><ResponsiveText text = {value}/></button>
            }
        } else {
            return html! {
                <button class = "cell">{""}</button>
            }
        }
    }
}
impl Component for Ticket{
    type Message = TicketMessage;
    type Properties = TicketProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Ticket{
            props,
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            TicketMessage::Clicked(number)=>{
                self.props.positioned_numbers = self.props.positioned_numbers.iter().map(|pn|{
                    if pn.number.clone() == number {
                        PositionedNumber{
                            row:pn.row.clone(),
                            column:pn.column.clone(),
                            claimed:true,
                            number:pn.number.clone()
                        }
                    } else {
                        pn.clone()
                    }
                }).collect();
            }
        }

        return true;
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let mut cell_props = vec![];
        for row in 0..3 {
            for column in 0..9 {
                let find = self.props.positioned_numbers.iter().find(|pn|{pn.row == row && pn.column == column});
                if let Some(found_pn) = find {
                    cell_props.push((row,column,Option::Some(found_pn.clone())));
                } else {
                    cell_props.push((row,column,Option::None ))
                }
            }
        }
        let callback = self.link.callback(|number:u8| TicketMessage::Clicked(number));
        let cell_renderer = |(row,column,pn):&(u8,u8,Option<PositionedNumber>)| html! {
            <Cell row = {row} column = {column} value = {pn} parent_callback = {callback.clone()} disabled = { self.props.disabled.clone() }/>
        };

        html! {
            <div class = "ticket">
                {for cell_props.iter().map(cell_renderer)}
            </div>
        }
    }
}
