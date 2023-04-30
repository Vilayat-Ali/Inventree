use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlInputElement, InputEvent};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub placeholder: String,
}

/// Controlled Text Input Component
#[function_component]
pub fn TextInput(props: &Props) -> Html {
    let handle_input = |e: Event| {
        let value = e
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        log!(val);
    };

    html! {
        <input type="text" placeholder={&props.placeholder} class="input input-bordered" onchange={} />
    }
}
