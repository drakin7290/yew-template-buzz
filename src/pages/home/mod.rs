pub mod style;

use style::{stylesheet, ClassStyle};
use stylist::yew::styled_component;
use yew::{html, Html, Properties};

use crate::layouts::main_layout::MainLayout;

use crate::core::route::Route;

use yew_router::prelude::Link;

#[derive(PartialEq, Properties)]
pub struct HomePageProps {}

#[styled_component(HomePage)]
pub fn home_page(props: &HomePageProps) -> Html {
    let HomePageProps {} = props;
    let stylesheet: ClassStyle = stylesheet();
    html! {
        <MainLayout title="Subwallet - Homepage">
            <div class={stylesheet.header}>
                <img src="/assets/abc.png"/>
            </div>
            <Link<Route> to={Route::Login}>{ "click here to go home" }</Link<Route>>
        </MainLayout>
    }
}
