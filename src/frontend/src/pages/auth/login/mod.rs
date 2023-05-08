use yew::{
    prelude::{function_component, html, Html},
    use_state, Callback,
};

use crate::layout::website::WebsiteLayout;

#[function_component]
pub fn Login() -> Html {
    html! {
        <WebsiteLayout>
            <div class="flex flex-col items-center justify-center h-screen">
                <div class="bg-white dark:bg-gray-700 p-10 rounded-lg shadow-lg">
                    <h1 class="text-2xl font-semibold mb-6">{"Login"}</h1>
                    <form>
                    <div class="mb-4">
                        <label
                        class="block text-gray-700 dark:text-white font-bold mb-2"
                        for="username"
                        >
                        {"Username"}
                        </label>
                        <input
                        class="appearance-none border rounded w-full py-2 px-3 text-gray-700 dark:text-gray-200 leading-tight focus:outline-none focus:shadow-outline"
                        id="username"
                        type="text"
                        placeholder="Username"
                        />
                    </div>
                    <div class="mb-4">
                        <label
                        class="block text-gray-700 dark:text-white font-bold mb-2"
                        for="password"
                        >
                        {"Password"}
                        </label>
                        <input
                        class="appearance-none border rounded w-full py-2 px-3 text-gray-700 dark:text-gray-200 mb-3 leading-tight focus:outline-none focus:shadow-outline"
                        id="password"
                        type="password"
                        placeholder="Password"
                        />
                    </div>
                    <div class="flex items-center justify-between">
                        <button
                        class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                        type="button"
                        >
                        {"Sign In"}
                        </button>
                        <a class="inline-block align-baseline font-bold text-sm text-blue-500 hover:text-blue-800" href="#">
                        {"Forgot Password?"}
                        </a>
                    </div>
                    </form>
                </div>
                <div class="mt-4">
                    <div class="toggle inline-flex items-center justify-center rounded-full w-14 h-8 transition-all duration-300 ease-in-out bg-gray-400 dark:bg-gray-500">
                    <input
                        class="toggle-checkbox absolute block w-6 h-6 rounded-full bg-white border-4 appearance-none cursor-pointer"
                        type="checkbox"
                    />
                    <label class="toggle-label block overflow-hidden h-8 rounded-full bg-gray-800 cursor-pointer"></label>
                    </div>
                </div>
            </div>
        </WebsiteLayout>
    }
}
