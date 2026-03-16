use crate::struct_ex::struct_example;
use crate::enum_ex::enum_example;
use crate::data_types::data_types_example;
use crate::pointers_ex::pointers_example;
mod struct_ex;
mod enum_ex;
mod data_types;
mod pointers_ex;

fn main() {
    println!("Data Types:");
    println!("---------------------");
    data_types_example();

    println!("\n");
    println!("Struct:");
    println!("---------------------");
    struct_example();

    println!("\n");
    println!("Enum:");
    println!("---------------------");
    enum_example();
    println!("\n");

    println!("Pointers:");
    println!("---------------------");
    pointers_example();
    println!("\n");

}