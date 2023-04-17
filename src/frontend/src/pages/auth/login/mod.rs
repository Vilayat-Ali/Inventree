use yew::prelude::{function_component, html, Html};

use crate::layout::website::WebsiteLayout;
use crate::pages::auth::form::login_form::LoginForm;

#[function_component]
pub fn Login() -> Html {
    let mut email = String::new();
    let mut password = String::new();

    html! {
        <WebsiteLayout>
            <LoginForm email={email} password={password} />
        </WebsiteLayout>
    }
}
