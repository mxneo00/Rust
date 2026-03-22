// A demo of using Rust to discuss ownership and borrowing concepts.
// Ownership is a set of rules that govern how a Rust program manages memory. 
// It is a key feature of Rust that allows it to ensure memory safety without needing a garbage collector.

// Rust utilizes both the stacka and the heap for memory management.
// The stack is a region of memory that stores values in a last-in, first-out manner.
// The heap is a region of memory that allows for dynamic allocation and deallocation of memory at runtime.
// There are different situations where you would want to use the stack or the heap.

// Ownership rules:
// 1. Each value in Rust has a variable that is called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

fn main() {
    {
        let s = "hello"; // s is a string literal, which is stored on the stack
        println!("{}", s); // s is valid here
    }
    // s is not valid here, it has gone out of scope

    // To make a string that is stored on the heap, we can use the String type:
    {
        // This also allows for mutability, which is not possible with string literals.
        let mut s = String::from("hello");
        s.push_str(", world!"); // push_str() appends a string slice to a String
        println!("{}", s); 
    }

    // Multiple ownership can be achieved using references and borrowing:
    // This is also known as a shallow copy, where both variables point to the same data on the heap, and only one variable is the owner of that data.
    {
        // In rust since s1 is a String being stored on the heap, we cant simply perform let s2 = s1; because that would move the ownership of the data from s1 to s2, making s1 invalid.
        let s1 = String::from("hello");
        let s2 = &s1; // s2 is a reference to s1
        println!("s1: {}, s2: {}", s1, s2); // both s1 and s2 are valid here
    }

    // An alternative to references is using the clone() method to create a deep copy of the data:
    // A deep copy means that the data is duplicated, and both variables own their own copy of the data.
    {
        let s1 = String::from("hello");
        let s2 = s1.clone(); // s2 is a deep copy of s1
        println!("s1: {}, s2: {}", s1, s2); // both s1 and s2 are valid here
    }

    // For data that is stored on the stack, such as integers, the ownership rules are different:
    {
        // Since integers are stored on the stack, they are copied by default when assigned to another variable. This means that both variables are valid and own their own copy of the data.
        let x = 5; // x is an integer, which is stored on the stack
        let y = x; // y is a copy of x, both x and y are valid here
        println!("x: {}, y: {}", x, y);
    }

    // Ownership and functions:
    {
        // When a variable is passed to a function, the ownership of the variable is transferred to the function. 
        fn takes_ownership(s: String) {
            println!("{}", s);
        }

        let s = String::from("hello");
        takes_ownership(s); // s's ownership is moved to the function
        // println!("{}", s); // this would cause an error because s is no longer valid here
    }

    // Returning values from functions also involves ownership:
    {
        fn gives_ownership() -> String {
            let s = String::from("hello");
            s // s's ownership is moved to the caller
        }

        let s1 = gives_ownership(); // s1 now owns the string returned by the function
        println!("{}", s1); // s1 is valid here
    }

}
