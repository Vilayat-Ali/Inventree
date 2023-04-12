use crate::State;
use yew::prelude::{function_component, html, Html, Properties};
use yewdux::prelude::*;

#[derive(PartialEq)]
pub struct Item {
    pub title: String,
    pub href: &'static str,
    pub icon: &'static str,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub items: Option<Vec<Item>>,
}

#[function_component]
pub fn Sidebar(_props: &Props) -> Html {
    let (state, _dispatch) = use_store::<State>();

    html! {
        if state.menu_state {
            <div class="drawer drawer-mobile">
                <div class="drawer-side border-r-4 border-indigo-500">
                        <ul class="menu p-4 w-80 bg-base-100 text-base-content">
                            <li><a>{"Sidebar Item 1"}</a></li>
                            <li><a>{"Sidebar Item 2"}</a></li>
                        </ul>
                </div>
            </div>
        }
    }
}
