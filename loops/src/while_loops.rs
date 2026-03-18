
// This function demonstrates the use of while loops in Rust.
// while loops execute a block of code as long as a condition is true
// do-while loops are not directly supported in Rust, but can be simulated using a loop and a break condition
pub fn while_loops() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("Liftoff!");
    // Simulating a do-while loop in Rust
    let mut number = 3;
    loop {
        println!("{}!", number);
        number -= 1;
        if number == 0 {
            break;
        }
    }
    println!("Liftoff!");

}