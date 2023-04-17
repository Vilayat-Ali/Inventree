use yew::prelude::{function_component, html, Html};

use crate::layout::website::WebsiteLayout;
use crate::pages::auth::form::signup_form::SignupForm;

#[function_component]
pub fn Signup() -> Html {

    html! {
        <WebsiteLayout>
            <SignupForm username={String::new()} email={String::new()} password={String::new()} />
        </WebsiteLayout>
    }
}
