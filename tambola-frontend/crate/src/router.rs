use yew_router::{switch::Permissive, Switch};
#[derive(Switch, Debug, Clone)]
pub enum TambolaRouter{
    #[to = "/!"]
    RootPath,
    #[to = "/game/{id}"]
    Game(String),
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
}