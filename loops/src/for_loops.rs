
// This function demonstrates the use of for loops in Rust.
// for loops iterate over a range or an iterator
// In rust for loops are the most commonly used loops
// This is because of the safety and conciseness they provide
// They are even preferred over while loops in many cases because they reduce the risk of infinite loops and off-by-one errors.
pub fn for_loops() {
    for i in 0..5 {
        println!("Iteration {}", i);
    }
}