use yew::prelude::{function_component, html, Html};
use yew_router::prelude::Link;
use yewdux::prelude::use_store;

use crate::app::Route;
use crate::components::navbar::NAV_ITEMS_WEBSITE;
use crate::State;

#[function_component]
pub fn Navbar() -> Html {
    let (_state, dispatch) = use_store::<State>();

    let toggle_theme =
        dispatch.reduce_mut_callback(|state| state.is_dark_theme = !state.is_dark_theme);

    let nav_item: Vec<Html> = {
        let mut v: Vec<Html> = Vec::with_capacity(NAV_ITEMS_WEBSITE.len());
        for menu_item in NAV_ITEMS_WEBSITE.into_iter() {
            v.push(html! {
                <>
                    <a href={menu_item[1]} class="p-2 btn btn-ghost mx-2">{menu_item[0]}</a>
                </>
            })
        }
        v
    };

    html! {
        <div>
            <div class="navbar bg-base-100 border-1 border-gray-600">
                <div class="navbar-start">
                    <Link<Route> to={Route::Home}><a class="btn btn-ghost normal-case text-xl">{"Inventree"}</a></Link<Route>>
                </div>
                <div class="navbar-center">
                    <div class="d-flex flex-row items-center">
                        {nav_item}
                    </div>
                </div>
                <div class="navbar-end">
                    <input type="checkbox" class="toggle" onchange={toggle_theme} />
                    <button class="btn btn-ghost btn-circle">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" /></svg>
                    </button>
                    <button class="btn btn-ghost btn-circle">
                    <div class="indicator">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" /></svg>
                        <span class="badge badge-xs badge-primary indicator-item"></span>
                    </div>
                    </button>
                </div>
            </div>
        </div>
    }
}
