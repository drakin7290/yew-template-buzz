use yew_router::Routable;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/auth/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}
