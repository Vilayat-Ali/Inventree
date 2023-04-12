use yew::prelude::{function_component, html, Html};

use crate::layout::website::WebsiteLayout;
use crate::pages::auth::form::signup_form::SignupForm;

#[function_component]
pub fn Signup() -> Html {
    let mut username = String::new();
    let mut email = String::new();
    let mut password = String::new();

    html! {
        <WebsiteLayout>
            <SignupForm username={username} email={email} password={password} button_name={String::from("Sign Up")}/>
        </WebsiteLayout>
    }
}
