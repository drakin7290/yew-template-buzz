pub mod style;

use style::{stylesheet, ClassStyle};
use stylist::yew::styled_component;
use yew::{html, Html, Properties};

use crate::layouts::main_layout::MainLayout;

#[derive(PartialEq, Properties)]
pub struct LoginPageProps {}

#[styled_component(LoginPage)]
pub fn login_page(props: &LoginPageProps) -> Html {
    let LoginPageProps {} = props;
    let stylesheet: ClassStyle = stylesheet();
    html! {
        <MainLayout title="Subwallet - Login">
            <div class={stylesheet.header}>
                <img src="/assets/abc.png"/>
            </div>
        </MainLayout>
    }
}
