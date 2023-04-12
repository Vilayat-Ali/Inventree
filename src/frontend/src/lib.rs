pub mod app;
pub mod components;
pub mod layout;
pub mod pages;
pub mod utils;

// wee_alloc for allocator for WASM
// reduce the size of the wasm bundle
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// yewdux state
use yewdux::prelude::*;

#[derive(Default, Clone, PartialEq, Eq, Store)]
pub struct State {
    pub menu_state: bool,
    pub is_dark_theme: bool,
}
