// Data Types in Rust

// Four primary scalar types in Rust: integers, floating-point numbers, booleans, and characters.
// Integer types in Rust: i8, i16, i32(default), i64, i128, isize (signed) and u8, u16, u32, u64, u128, usize (unsigned)
// Float types in Rust: f32 (32-bit) and f64 (64-bit, default)
pub fn data_types_example() {
    let integer: i32 = 42; // 32-bit signed integer
    let float: f64 = 3.14; // 64-bit floating point number
    let boolean: bool = true; // Boolean value
    let character: char = 'A'; // Character value
    println!("Integer: {}", integer);
    println!("Float: {}", float);
    println!("Boolean: {}", boolean);
    println!("Character: {}", character);
}