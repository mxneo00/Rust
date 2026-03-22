// A demo of how to organize a Rust project with multiple modules and files.

// A package is a collection of Rust code. A package can contain multiple crates, and a crate can contain multiple modules.
// A crate is a tree of modules that produces a library or an executable. A crate can be a binary crate (an executable) or a library crate (a reusable library).
// A module is a way to organize code within a crate. A module can contain functions, structs, enums, and other items.

// Run cargo new rust_demo to create a new Rust project named rust_demo this is also considered the package.
// The typical structure of a Rust project looks like this:
// rust_demo/
// ---Cargo.toml
// ---src/
// -------main.rs
// -------utils.rs
// -------math.rs

// This is how we recieve functions from other files in the same project. 

mod utils; // Declare the utils module, which is defined in utils.rs
mod math; // Declare the math module, which is defined in math.rs

// This is a shortcut to use the add function from the math module without having to specify the full path every time we want to use it.
use math::add; 

// The main function is considered the binary crate of the project, and it can use the functions defined in the utils and math modules.
fn main() {
    // Use a function from the utils module
    let greeting = utils::greet("Alice");
    println!("{}", greeting);

    // Use a function from the math module
    let result = add(5, 3);
    println!("5 + 3 = {}", result);
    // We can also use the full path to the add function if we want to but it's not necessary since we have already imported it with the use statement.
    let result2 = math::add(10, 20); 
    println!("10 + 20 = {}", result2);

    // When using a nested module, we need to specify the full path to the function we want to use.
    let factorial_result = math::advanced::factorial(5);
    println!("5! = {}", factorial_result);
}
