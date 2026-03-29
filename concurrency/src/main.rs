// Concurrency demos
// Safe and efficient concurrency is one Rust's main goals
// The rust team discovered that the ownership and type systems were powerful tools for this.
// Fearless concurrency
// this essentially means that Rust allows you to write concurrent code without fear of data races
// In rust many concurrency issues are caught at compile time rather than at runtime

mod race_condition;

fn main() {
    race_condition::race_condition_demo();
}