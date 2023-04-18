mod button;
mod input;

use gloo_console::log;
use yew::prelude::*;
use yew_router::prelude::Link;
use yewdux::prelude::*;

use crate::{
    app::Route,
    pages::auth::form::signup_form::{button::FormButton, button::PayloadData, input::TextInput},
    {SignupDetails, State},
};

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[function_component]
pub fn SignupForm(props: &Props) -> Html {
    let (state, dispatch) = use_store::<State>();
    let mut payload_data: PayloadData = PayloadData::new(String::new(), String::new());
    let mut signup_details: SignupDetails = SignupDetails::default();

    let mut username_callback_data_received = Callback::from(|received_input: String| {
        signup_details.username = received_input;
        //* Log recieved callback action
        log!(received_input);
    });

    let mut email_callback_data_received = Callback::from(|received_input: String| {
        signup_details.email = received_input;
        //* Log recieved callback action
        log!(received_input);
    });

    let mut password_callback_data_received = Callback::from(|received_input: String| {
        signup_details.password = received_input;
        //* Log recieved callback action
        log!(received_input);
    });

    let handle_click = Callback::from(move |_| {
        let SignupDetails {
            username,
            email,
            password,
        } = signup_details;

        if username.len() == 0 || email.len() == 0 || password.len() == 0 {
            log!("Bhaiyya fields toh bhar lo!");
        } else {
            log!(format!("{} with email {} is registered", username, email));
        }
    });

    use_effect(move || {
        // clean up
        || {
            dispatch.reduce_mut_callback(|s| s.signup_details = SignupDetails::default());
        }
    });

    html! {
      <>
        <div class="hero min-h-screen bg-base-200">
            <div class="hero-content flex-col lg:flex-row-reverse">
                <div class="text-center lg:text-left">
                <h1 class="text-5xl font-bold">{"Sign Up!"}</h1>
                <p class="py-6">{"Welcome to Inventree! Please fill in the form below to create your account. Once you have completed the form, click the 'Sign Up' button to proceed. By creating an account, you'll be able to enjoy all the features and benefits of our inventory management system. Already have an account? You can log in using the 'Login' link at the top of the page."}</p>
                </div>
                <div class="card flex-shrink-0 w-full max-w-sm shadow-2xl bg-base-100">
                <div class="card-body">
                <div class="form-control">
                    <label class="label">
                        <span class="label-text">{"Username"}</span>
                    </label>
                    <TextInput placeholder={String::from("Username")} value={props.username.clone()} on_change={username_callback_data_received} />
                    </div>
                    <div class="form-control">
                    <label class="label">
                        <span class="label-text">{"Email"}</span>
                    </label>
                    <TextInput placeholder={String::from("Email")} value={props.email.clone()} on_change={email_callback_data_received} />
                    </div>
                    <div class="form-control">
                    <label class="label">
                        <span class="label-text">{"Password"}</span>
                    </label>
                    <TextInput placeholder={String::from("Password")} value={props.password.clone()} on_change={password_callback_data_received} />
                    <label class="label">
                    <Link<Route> to={Route::Login}><a class="label-text-alt link link-hover">{"Have an account? Login"}</a></Link<Route>>
                    </label>
                    </div>
                    <div class="form-control mt-6">
                    <FormButton button_text="Sign up" payload={payload_data} handle_click={handle_click}/>
                    </div>
                </div>
                </div>
            </div>
        </div>
      </>
    }
}
