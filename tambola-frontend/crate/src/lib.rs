#![recursion_limit = "1024"]

#[macro_use]
extern crate cfg_if;

extern crate wasm_bindgen;
extern crate web_sys;
extern crate yew;
extern crate yew_router;
extern crate yew_styles;
extern crate yewtil;
extern crate failure;
extern crate serde;
extern crate serde_json;
extern crate anyhow;
extern crate tambola_lib;
extern crate uuid;

use wasm_bindgen::prelude::*;
pub mod agents;
mod app;
mod router;
mod pages;
mod components;
use app::App;

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        fn set_panic_hook() {}
    }
}

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn run() {
    // If the `console_error_panic_hook` feature is enabled this will set a panic hook, otherwise
    // it will do nothing.
    set_panic_hook();

    yew::start_app::<App>();
}
