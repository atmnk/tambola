use yew::prelude::*;
/// Test page
pub struct TestPage;
use crate::components::game::Ticket;
use tambola_lib::game::get_random_ticket_numbers;

impl Component for TestPage {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        TestPage {}
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let numbers = get_random_ticket_numbers();
        let numbers1 = get_random_ticket_numbers();
        html! {
            <div class="test">
                <div class="user"><Ticket positioned_numbers = {numbers} disabled = {false}/></div>
                <div class = "announced"><Ticket positioned_numbers = {numbers1} disabled= {true}/></div>
            </div>
        }
    }
}

