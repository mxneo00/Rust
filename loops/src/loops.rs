
// This function demonstrates an infinite loop using the `loop` keyword in Rust.
// The loop keyword creates an infinite loop
// You can break out of the loop using the `break` keyword or ctrl+C in the terminal
pub fn loops() {
    loop {
        println!("This is an infinite loop");
        break;
    }
    let mut counter = 0;
    // This loop demonstrates how to return a value from a loop using the `break` keyword.
    // If you have loops within loops break and continue apply to the innermost loop
    let result = loop {
        counter += 1;
        if counter == 5 {
            println!("Counter is {}", counter);
            break counter * 2;
        }
    };
    println!("The result is {}", result);
    // Loop labels can be used to specify which loop a `break` or `continue` statement applies to.
    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            println!("Entered the inner loop");
            break 'outer;
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");

}