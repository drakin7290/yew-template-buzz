pub mod style;

use stylist::yew::styled_component;

use yew::{html, props, Callback, Children, Html, Properties};

// use crate::log::log;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(true)]
    pub is_loading: bool,
    #[prop_or_default]
    pub on_click: Callback<String>,
    #[prop_or_default]
    pub children: Children,
}

#[styled_component(HelloWord)]
pub fn hello_word(props: &Props) -> Html {
    let ko: Props = props! {
        Props {
            is_loading: false,
        }
    };
    let stylesheet: stylist::Style = style::stylesheet();
    props.on_click.emit("I loaded".to_owned());
    html! {
        <>{"Am I loading? - "}{ko.is_loading.clone()}{props.is_loading.clone()}
            <p class={stylesheet}>{"Click"}</p>
            {for props.children.iter()}
        </>
    }
}
