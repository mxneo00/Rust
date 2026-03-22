// This file defines the math module, which contains mathematical functions that can be used in main.rs or other modules.
// The functions in this module are public, so they can be accessed from other modules.

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Rust also allows for modules to be nested, so we can define a submodule for more specific math functions.
pub mod advanced {
    pub fn factorial(n: u32) -> u32 {
        if n == 0 {
            1
        } else {
            n * factorial(n - 1)
        }
    }
}