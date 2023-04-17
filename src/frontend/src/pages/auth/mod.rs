pub mod form;
pub mod login;
pub mod signup;

use crate::app::Route;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_authorized: bool,
    pub children: Children,
}

#[function_component]
pub fn AuthProtected(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    let Props {
        is_authorized,
        children,
    } = props;

    use_effect_with_deps(
        move |_| {
            if !(*is_authorized) {
                navigator.push(&Route::Login);
            }
        },
        props.is_authorized,
    );

    html! {
        <div>
            {children.iter().collect::<Html>()}
        </div>
    }
}
