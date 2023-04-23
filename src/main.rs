mod app;
mod components;
mod containers;
mod contexts;
mod core;
mod hooks;
mod layouts;
mod log;
mod pages;
mod utils;

use crate::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
