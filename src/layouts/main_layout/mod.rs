pub mod style;

use style::{stylesheet, ClassStyle};
use stylist::yew::styled_component;
use yew::{html, Children, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct MainLayoutProps {
    #[prop_or("Subwallet WASM".to_owned())]
    pub title: String,
    pub children: Children,
}

#[styled_component(MainLayout)]
pub fn main_layout(props: &MainLayoutProps) -> Html {
    let MainLayoutProps { children, title } = props;

    ////////
    let document_head = gloo::utils::document()
        .head()
        .expect("head element to be present");
    let title_element = document_head
        .query_selector("title")
        .expect("to find a title element")
        .expect("to find a title element");
    title_element.set_text_content(Some(title));
    //////// --- end Update

    let stylesheet: ClassStyle = stylesheet();
    html! {
        <div class={stylesheet.header}>
            { for children.iter() }
        </div>
    }
}
