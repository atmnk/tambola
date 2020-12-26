use yew::prelude::*;
use yew_styles::layouts::{
    container::{Container, Direction, Wrap},
    item::{Item, ItemLayout},
};
use yew_styles::layouts::item::AlignSelf;


pub mod new_game;
pub mod game;
pub enum NameConnectMessage{
    UpdatedName(String),
    Submitted,
}
pub struct NameConnect{
    name:String,
    link:ComponentLink<Self>,
    props:NameConnectProps
}
#[derive(Clone,Properties)]
pub struct NameConnectProps{
    label:String,
    onsubmit:Callback<String>
}
impl Component for NameConnect{
    type Message = NameConnectMessage;
    type Properties = NameConnectProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self{
            name:"".to_string(),
            link,
            props
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            NameConnectMessage::UpdatedName(name)=>{
                self.name = name;
                false
            },
            Self::Message::Submitted=>{
                self.props.onsubmit.emit(self.name.clone());
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let onkeypress = self.link.batch_callback(|event:KeyboardEvent| {
            if event.key() == "Enter" {
                vec![Self::Message::Submitted]
            } else {
                vec![]
            }
        });
        html! {
        <Container direction=Direction::Column wrap=Wrap::Wrap class_name="component name-connect">
                <Item layouts=vec!(ItemLayout::ItXs(12)) align_self=AlignSelf::Stretch>
                    <input
                    class="name"
                    placeholder="Your Name",
                    value = &self.name
                    oninput=self.link.callback(|e: InputData| NameConnectMessage::UpdatedName(e.value))
                    onkeypress=onkeypress
                    />
                </Item>
                <Item layouts=vec!(ItemLayout::ItXs(12)) align_self=AlignSelf::Center>
                    <button class = "start" onclick = self.link.callback(|_|Self::Message::Submitted)>{{&self.props.label}}</button>
                </Item>
            </Container>
        }
    }
}