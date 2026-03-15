// Constant: 
// Uppercase with underscores, immutable, and must be annotated with a type
const BANK_NAME: &str = "Rust Bank";

fn main() {
    println!("Welcome to {}!", BANK_NAME); // Retrieve constant value from global scope

    // Variable:
    // let keyword is used to declare variables in Rust. 
    // - `account_name` is immutable (cannot be changed after assignment) 
        // this is the default in Rust and has a type inferred from the assigned value (string slice `&str`).
    // - `balance` is mutable (can be changed after assignment) and has a type annotation of `f64` (floating-point number)
    let account_name = "Alice"; // Variable
    let mut balance: f64 = 1000.0; // Mutable variable with type annotation

    println!("{}'s initial balance: ${:.2}", account_name, balance);

    let deposit = 250.0;
    // Update the balance by adding the deposit amount. This is possible because `balance` is mutable.
    balance += deposit;
    println!("{} deposited ${:.2}. New balance: ${:.2}", account_name, deposit, balance);

    // Scope example: Withdraw money in a new scope
    // This block creates a new scope for the withdrawal operation.
    // Variables declared inside this block (like `withdrawal`) are not accessible outside of it.
    {
        let withdrawal = 300.0;
        if withdrawal <= balance {
            balance -= withdrawal;
            println!("{} withdrew ${:.2}. New balance: ${:.2}", account_name, withdrawal, balance);
        } else {
            println!("{} attempted to withdraw ${:.2}, but insufficient funds! Current balance: ${:.2}", account_name, withdrawal, balance);
        }
    }

    // shadowing example: Apply interest to the balance
    // Shadowing allows us to reuse the variable name `balance` for a new value, which is the result of applying interest.
    let balance = balance * 1.05; // Apply 5% interest
    println!("After applying interest, {}'s balance: ${:.2}", account_name, balance);
}
