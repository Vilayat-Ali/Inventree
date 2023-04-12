use yew::prelude::{function_component, html, Html};

use crate::layout::dashboard::DashboardLayout;

#[function_component]
pub fn Dashboard() -> Html {
    html! {
        <DashboardLayout>
            <h1>{"Dashboard"}</h1>
        </DashboardLayout>
    }
}
