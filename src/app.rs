use crate::components::hello_word::HelloWord;
use crate::log::*;
use web_sys::console;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    log("Hello rust");
    console::log_1(&format!("hello, alo {}", 1).into());
    let cb: Callback<String> = Callback::from(move |msg: String| log(&msg));
    html! {
        <>
        <button id="jett" class={classes!("connect")}>{"Connect Dapp"}</button>
        <HelloWord is_loading={true} on_click={cb}>
            <p>{"Hello Guys"}</p>
        </HelloWord>
        </>
    }
}
