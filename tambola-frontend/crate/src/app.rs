use yew::prelude::*;
use router::TambolaRouter;
use yew_router::{prelude::*, route::Route, switch::Permissive};
use pages::tambola::home::Home;
use pages::tambola::game::GamePage;
use pages::tambola::TambolaPage;


pub struct App;
impl Component for App{
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <Router<TambolaRouter, ()>
                    render = Router::render(|switch: TambolaRouter | {

                        match switch {
                            TambolaRouter::RootPath => html!{
                                <TambolaPage><Home/></TambolaPage>
                            },
                            TambolaRouter::Game(id) => {
                                html!{
                                <TambolaPage><GamePage id = id/></TambolaPage>
                            }},
                            TambolaRouter::PageNotFound(Permissive(None)) => html!{"Page not found"},
                            TambolaRouter::PageNotFound(Permissive(Some(missed_route))) => html!{format!("Page '{}' not found", missed_route)}
                        }
                    } )
                    redirect = Router::redirect(|route: Route<()>| {
                        TambolaRouter::PageNotFound(Permissive(Some(route.route)))
                    })
                />
            </div>
        }
    }
}