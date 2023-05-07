use yew::prelude::{function_component, html, Html};

use crate::layout::website::WebsiteLayout;

#[function_component]
pub fn Login() -> Html {
    html! {
        <WebsiteLayout>
            <h1>{"Login cange"}</h1>
        </WebsiteLayout>
    }
}
