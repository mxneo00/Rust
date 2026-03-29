// Race Condition demo
use std::sync::{Arc, Mutex};
use std::thread;

// Race condition: correctness of a program depends on the relative timing of threads
// Race conditions in rust can be prevented using synchronization primitives like Mutex and Arc

// Simulated Race conditon
pub fn simulated_race_condition() {
    let mut handles = vec![];
    let mut counter = 0;

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    // This will likely print a number less than 10000 due to a race condition
    println!("Final counter: {}", counter);
}

pub fn race_condition_fixed() {
    // Using Arc and Mutex are tools to safely share and modify data across multiple threads
    // Arc stands for Atomic Reference Counted, it allows multiple ownership of the same data across threads
    // Mutex is a mutual exclusion primitive useful for protecting shared data

    // Here we use Arc to share ownership of the counter across threads
    // and Mutex to ensure that only one thread can modify the counter at a time
    let counter = Arc::new(Mutex::new(0u32));
    let mut handles = vec![];

    for _ in 0..10 {
        // Instead of moving or just copying the counter, we clone the Arc to increase the reference count
        // This ensures that each thread has its own reference to the same counter (Thread-local storage)
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter: {}", counter.lock().unwrap());

}


pub fn race_condition_demo() {
    simulated_race_condition();
}