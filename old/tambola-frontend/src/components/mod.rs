use yew::prelude::*;

pub mod nav;
pub mod game;
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
