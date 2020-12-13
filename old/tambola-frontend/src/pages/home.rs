use yew::prelude::*;
use yew::services::DialogService;

pub struct Home;
impl Component for Home{
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Home{

        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        true
    }

    fn view(&self) -> Html {
        html!{
            <NewGame name={""}/>
        }
    }
}
pub struct NewGame{
    props:NewGameProps,
    link:ComponentLink<Self>
}
#[derive(Properties,Clone)]
pub struct NewGameProps{
    name:String,
}
pub enum NewGameMessage{
    UpdatedName(String),
    StartNewGame,
}
impl Component for NewGame{
    type Message = NewGameMessage;
    type Properties = NewGameProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        NewGame{
            props,
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            NewGameMessage::UpdatedName(name)=>{
                self.props.name = name;
                false
            },
            NewGameMessage::StartNewGame=>{
                DialogService::alert("Thanks for Submitting");
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        unimplemented!()
    }

    fn view(&self) -> Html {
        let onkeypress = self.link.batch_callback(|event:KeyboardEvent| {
            if event.key() == "Enter" {
                vec![NewGameMessage::StartNewGame]
            } else {
                vec![]
            }
        });
        html! {
            <div class="new-game">
            <div class="form">
                <input
                    class="name"
                    placeholder="Your Name"
                    value=&self.props.name
                    oninput=self.link.callback(|e: InputData| NewGameMessage::UpdatedName(e.value))
                    onkeypress=onkeypress
                />
                <button class = "start" onclick = self.link.callback(|_|NewGameMessage::StartNewGame)>{"Start New Game"}</button>
            </div>
            </div>
        }
    }
}