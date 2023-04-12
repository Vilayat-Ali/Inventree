use yew::prelude::{function_component, html, Html};

use crate::layout::website::WebsiteLayout;

#[function_component]
pub fn Signup() -> Html {
    html! {
        <WebsiteLayout>
            <h1>{"Signup"}</h1>
        </WebsiteLayout>
    }
}
