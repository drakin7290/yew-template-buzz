mod app;
mod components;
mod log;

use crate::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
