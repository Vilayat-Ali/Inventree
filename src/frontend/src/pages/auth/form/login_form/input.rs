use gloo_console::log;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub placeholder: String,
}

/// Controlled Text Input Component
#[function_component]
pub fn TextInput(props: &Props) -> Html {
    let onchange = Callback::from(|e: Event| {
        let value: String = e
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();

        log!(format!("{} : {}", props.placeholder.clone(), value));
    });

    html! {
        <>
            <input type="text" placeholder={props.placeholder.clone()} class="input input-bordered" {onchange} />
        </>
    }
}
