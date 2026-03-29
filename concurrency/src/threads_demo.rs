// Rust threads demo
use std::thread;

fn threads_demo() {
    // In order to create a new thread, we use the thread::spawn function
    // The thread::spawn is passed a closure that contains the code to be executed in the new thread
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
    }
    // The join method is used to wait for the thread to finish
    // The return type of thread::spawn is a JoinHandle<T>
    // The join method returns a Result<T, E> where T is the return type of the closure
    // Because this is called after the main threads for loop it will wait until the spawned is finshed before ending.
    handle.join().unwrap();

}

fn move_closure_demo() {
    let v = vec![1, 2, 3];

    // The move keyword is used to transfer ownership of v into the closure
    // Without this v would just be borrowed which is not allowed
    // This is because Rust cant tell how long the spawned thread will run 
    // and therefore cannot allow a reference to v to be used in the closure
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}