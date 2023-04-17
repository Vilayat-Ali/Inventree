use yew::prelude::*;
use yew_router::prelude::*;

use crate::State;
use yewdux::prelude::use_store_value;

// importing pages
use crate::pages::{
    auth::{login::Login, signup::Signup, AuthProtected},
    dashboard::Dashboard,
    home::Home,
    not_found::NotFound,
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/dashboard")]
    Dashboard,
    #[at("/auth/login")]
    Login,
    #[at("/auth/signup")]
    Signup,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn handle_routing(
    route: Route,
    is_user_authorized: bool,
) -> Html {
    let state = use_store_value::<State>();

    match route {
        Route::Home => html! {<Home />},
        Route::Dashboard => html! {
            <AuthProtected is_authorized={is_user_authorized}>
                <Dashboard />
            </AuthProtected>
        },
        Route::Login => html! {<Login />},
        Route::Signup => html! {<Signup />},
        Route::NotFound => html! {<NotFound />},
    }
}

#[function_component]
pub fn App() -> Html {
    let state = use_store_value::<State>();

    html! {
        if state.is_dark_theme {
            <div data-theme="dark">
                <BrowserRouter>
                    <Switch<Route> render={handle_routing(state.is_user_authorized)} />
                </BrowserRouter>
            </div>
        } else {
            <div data-theme="light">
                <BrowserRouter>
                    <Switch<Route> render={handle_routing(state.is_user_authorized)} />
                </BrowserRouter>
            </div>
        }
    }
}
