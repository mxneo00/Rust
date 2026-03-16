pub fn pointers_example() {
    // Pointers in Rust
    // References allow you to refer to some value without taking ownership of it
    // Mutable references allow you to modify the value they point to
    println!("Pointers Example:");
    let x = 5;
    let y = &x; // y is a reference to x
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    let mut z = 10;
    let z_ref = &mut z; // z_ref is a mutable reference to z
    *z_ref += 5; // modify the value of z through the reference
    println!("The value of z is: {}", z);

    // Smart pointers in Rust
    // Smart pointers are data structures that act like a pointer but also have additional metadata and capabilities
    // Examples include Box<T>, Rc<T>, and RefCell<T>
    println!("Smart Pointers Example:");
    let b = Box::new(5); // Box<T> allocates memory on the heap
    println!("The value of b is: {}", b);
}