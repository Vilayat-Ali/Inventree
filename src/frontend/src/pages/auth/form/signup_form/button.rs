use gloo_console::log;
use yew::prelude::*;
#[derive(PartialEq, Clone)]
pub struct PayloadData {
    pub email_data: String,
    pub password_data: String,
}

impl PayloadData {
    pub fn new(
        email_data: String,
        password_data: String,
    ) -> Self {
        Self {
            email_data,
            password_data,
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub button_text: String,
    pub payload: PayloadData,
    pub handle_click: Callback<MouseEvent>,
}

#[function_component]
pub fn FormButton(props: &Props) -> Html {
    let PayloadData {
        email_data,
        password_data,
    } = &props.payload;

    html! {
      <button class="btn btn-primary" onclick={props.handle_click.clone()}>{&props.button_text}</button>
    }
}
