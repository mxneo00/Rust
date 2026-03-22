// This file defines the utils module, which contains utility functions that can be used in main.rs or other modules.
// The functions in this module are public, so they can be accessed from other modules.
// By default Rust modules are private, so we need to use the pub keyword to make them public.

pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
