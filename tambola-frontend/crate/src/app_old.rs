use crate::pages::{About, Home};
use yew::prelude::*;
use yew_router::{prelude::*, route::Route, switch::Permissive, Switch};


pub struct App {
    link: ComponentLink<Self>,
}

#[derive(Switch, Debug, Clone)]
pub enum AppRouter {
    #[to = "/!"]
    RootPath,
    #[to = "/about/me"]
    AboutPath,
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
}


impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <Router<AppRouter, ()>
                    render = Router::render(|switch: AppRouter | {
                        match switch {
                            AppRouter::RootPath => html!{
                                <Home/>
                            },
                            AppRouter::AboutPath => html!{
                                <About/>
                            },
                            AppRouter::PageNotFound(Permissive(None)) => html!{"Page not found"},
                            AppRouter::PageNotFound(Permissive(Some(missed_route))) => html!{format!("Page '{}' not found", missed_route)}
                        }
                    } )
                    redirect = Router::redirect(|route: Route<()>| {
                        AppRouter::PageNotFound(Permissive(Some(route.route)))
                    })
                />
            </div>
        }
    }
}
