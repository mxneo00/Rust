const BANK_NAME: &str = "Rust Bank"; // Constant

fn main() {
    println!("Welcome to {}!", BANK_NAME); // Retrieve constant value from global scope

    let account_name = "Alice"; // Variable
    let mut balance: f64 = 1000.0; // Mutable variable with type annotation

    println!("{}'s initial balance: ${:.2}", account_name, balance);

    let deposit = 250.0;
    balance += deposit;
    println!("{} deposited ${:.2}. New balance: ${:.2}", account_name, deposit, balance);

    // Scope example: Withdraw money in a new scope
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
    let balance = balance * 1.05; // Apply 5% interest
    println!("After applying interest, {}'s balance: ${:.2}", account_name, balance);
}
