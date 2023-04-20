use yew::prelude::{function_component, html, Html};

use crate::layout::website::WebsiteLayout;
use crate::pages::auth::form::login_form::LoginForm;

#[function_component]
pub fn Login() -> Html {

    html! {
        <WebsiteLayout>
            <LoginForm email={String::new()} password={String::new()} />
        </WebsiteLayout>
    }
}
