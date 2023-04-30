use gloo_console::log;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;
use yewdux::dispatch;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub placeholder: String,
}

/// Controlled Text Input Component
#[function_component]
pub fn TextInput(props: &Props) -> Html {
    let (_state, dispatch) = use_store::<State>();

    let onchange = Callback::from(|e: Event| {
        let value: String = e
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();

        log!(format!("{}", value));
    });

    html! {
        <>
            <input type="text" placeholder={props.placeholder.clone()} class="input input-bordered" {onchange} />
        </>
    }
}
