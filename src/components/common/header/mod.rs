pub mod style;

use stylist::yew::styled_component;
use yew::{html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct HeaderProps {}

#[styled_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let HeaderProps {} = props;
    let stylesheet: stylist::Style = style::stylesheet();
    html! {
        <header class={stylesheet}></header>
    }
}
