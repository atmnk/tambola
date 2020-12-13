use yew::prelude::*;
pub mod tp;
pub mod home;
pub struct TambolaPage{
    props:TambolaPageProps
}
#[derive(Clone, Properties)]
pub struct TambolaPageProps{
    children:Children
}
impl Component for TambolaPage {
    type Message = ();
    type Properties = TambolaPageProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        TambolaPage{
            props
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        true
    }
    fn view(&self) -> Html {
        html! {
            <div class="page">
                { self.props.children.clone() }
            </div>
        }
    }
}

