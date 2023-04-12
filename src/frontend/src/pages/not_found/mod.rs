use yew::prelude::{function_component, html, Html};

use crate::layout::website::WebsiteLayout;

#[function_component]
pub fn NotFound() -> Html {
    html! {
        <WebsiteLayout>
            <h1>{"NotFound"}</h1>
        </WebsiteLayout>
    }
}
