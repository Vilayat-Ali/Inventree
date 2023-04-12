use yew::prelude::{function_component, html, Html};

#[function_component]
pub fn NotFound() -> Html {
    html! {
        <div>
            <h1>{"NotFound"}</h1>
        </div>
    }
}
