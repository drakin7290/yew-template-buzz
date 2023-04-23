// use crate::components::hello_word::HelloWord;
use crate::log::*;
use web_sys::console;
use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};

use crate::pages::auth::login::LoginPage;
use crate::pages::home::HomePage;

use crate::core::route::Route;

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage/>},
        Route::Login => html! {<LoginPage />},
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    log("Hello rust");
    console::log_1(&format!("hello, alo {}", 1).into());
    // let cb: Callback<String> = Callback::from(move |msg: String| log(&msg));
    let fallback: yew::virtual_dom::VNode = html! {<div>{"Loading..."}</div>};
    html! {
        <Suspense {fallback}>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </Suspense>
    }
}
