use gloo_console::log;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{HtmlInputElement, InputEvent};
use yew::{
    html::onchange::Event,
    prelude::{function_component, html, Html},
    use_state, Callback,
};

use crate::layout::website::WebsiteLayout;

#[function_component]
pub fn Login() -> Html {
    let email = use_state(|| String::new());
    let password = use_state(|| String::new());

    let oninput_email = Callback::from({
        let email = email.clone();
        move |input_event: InputEvent| {
            let target: HtmlInputElement = input_event.target().unwrap().dyn_into().unwrap();
            email.set(target.value());
        }
    });

    let oninput_password = Callback::from({
        let password = password.clone();
        move |input_event: InputEvent| {
            let target: HtmlInputElement = input_event.target().unwrap().dyn_into().unwrap();
            password.set(target.value());
        }
    });

    let handle_submit = Callback::from(move |_| {
        log!(format!("{}, {}", &*email.clone(), &*password.clone()));
    });

    html! {
        <WebsiteLayout>
            <div class="flex flex-col items-center justify-center h-screen">
                <div class="bg-white text-gray-800 dark:text-white dark:bg-gray-700 p-10 rounded-lg shadow-lg">
                    <h1 class="text-2xl font-semibold mb-6">{"Login"}</h1>
                    <form>
                    <div class="mb-4">
                        <label
                        class="block text-gray-800 dark:text-white font-bold mb-2"
                        for="Email"
                        >
                        {"Email"}
                        </label>
                        <input
                        oninput={oninput_email}
                        class="appearance-none border rounded w-full py-2 px-3 text-gray-700 dark:text-gray-200 leading-tight focus:outline-none focus:shadow-outline"
                        id="Email"
                        type="text"
                        placeholder="Email"
                        />
                    </div>
                    <div class="mb-4">
                        <label
                        class="block text-white dark:text-gray font-bold mb-2"
                        for="password"
                        >
                        {"Password"}
                        </label>
                        <input
                        class="appearance-none border rounded w-full py-2 px-3 text-gray-700 dark:text-gray-200 mb-3 leading-tight focus:outline-none focus:shadow-outline"
                        id="password"
                        type="password"
                        placeholder="Password"
                        oninput={oninput_password}
                        />
                    </div>
                    <div class="flex items-center justify-between">
                        <button
                        class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                        type="button"
                        onclick={handle_submit}
                        >
                        {"Log In"}
                        </button>
                    </div>
                    </form>
                </div>
            </div>
        </WebsiteLayout>
    }
}
