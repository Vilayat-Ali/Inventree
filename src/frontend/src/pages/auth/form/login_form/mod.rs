mod button;
mod input;

use gloo_console::log;
use yew::prelude::*;
use yew_router::prelude::Link;

use crate::{
    app::Route,
    pages::auth::form::login_form::{button::FormButton, input::TextInput},
};

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub email: String,
    pub password: String,
}

#[function_component]
pub fn LoginForm(props: &Props) -> Html {
    let handle_click = Callback::from(move |_| {
        log!("Hello");
        log!("Hello");
    });

    html! {
      <>
        <div class="hero min-h-screen bg-base-200">
            <div class="hero-content flex-col lg:flex-row-reverse">
                <div class="text-center lg:text-left">
                <h1 class="text-5xl font-bold">{"Login now!"}</h1>
                <p class="py-6">{"Welcome back! Please enter your login credentials below to access your account."}<br /> {"If you have forgotten your password, you can reset it using the 'Forgot Password' link. If you don't have an account yet, you can create one by clicking the 'Create Account' button."}</p>
                </div>
                <div class="card flex-shrink-0 w-full max-w-sm shadow-2xl bg-base-100">
                <div class="card-body">
                    <div class="form-control">
                    <label class="label">
                        <span class="label-text">{"Email"}</span>
                    </label>
                    <TextInput placeholder={String::from("Email")} />
                    </div>
                    <div class="form-control">
                    <label class="label">
                        <span class="label-text">{"Password"}</span>
                    </label>
                    <TextInput placeholder={String::from("Password")} />
                    <label class="label">
                        <Link<Route> to={Route::Signup}><a class="label-text-alt link link-hover">{"New User? Create Account"}</a></Link<Route>>
                    </label>
                    </div>
                    <div class="form-control mt-6">
                    <FormButton button_text="Login" handle_click={handle_click}/>
                    </div>
                </div>
                </div>
            </div>
        </div>
      </>
    }
}
