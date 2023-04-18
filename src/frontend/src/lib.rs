pub mod app;
pub mod components;
pub mod layout;
pub mod pages;
pub mod utils;

// wee_alloc for allocator for WASM
// reduce the size of the wasm bundle
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// console.log
pub use gloo_console::{error, info, log, table, warn};

// yewdux state
use yewdux::prelude::*;

/// importing the frontend envs
pub use security::{import_envs, FrontendConfig};

/// Login detail struct
#[derive(Clone, PartialEq, Eq, Default)]
pub struct LoginDetails {
    pub email: String,
    pub password: String,
}

impl LoginDetails {
    pub fn new(
        email: String,
        password: String,
    ) -> Self {
        Self { email, password }
    }
}

/// Signup detail struct
#[derive(Clone, PartialEq, Eq, Default)]
pub struct SignupDetails {
    pub email: String,
    pub username: String,
    pub password: String,
}

impl SignupDetails {
    pub fn new(
        email: String,
        username: String,
        password: String,
    ) -> Self {
        Self {
            username,
            email,
            password,
        }
    }
}

/// Equivalent to store in redux
#[derive(Default, Clone, PartialEq, Eq, Store)]
pub struct State {
    pub menu_state: bool,
    pub is_user_authorized: bool,
    pub is_dark_theme: bool,
    pub login_details: LoginDetails,
    pub signup_details: SignupDetails,
}
