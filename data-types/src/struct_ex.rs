// Structs in Rust
// Structs are used to create custom data types that group together related data
// They can have named fields, tuple-like fields, or no fields at all (unit-like structs)

struct User {
    first_name: String, // Used the owned String type for heap-allocated string data this allows the struct to own the string data
    middle_initial: char, 
    last_name: String,
    age: i32, 
    email: String,
}
// Tuple struct
// have the added meaning the struct provides over a regular tuple but don't have named fields
// useful when you want to give a tuple a name and make the tuple a different type from other tuples with the same types
struct Points(i32, i32, i32);
// Unit-like structs
// structs without any fields
// useful when you want to implement a trait on a type but don't need to store any data
struct AlwaysEqual;

pub fn struct_example() {
    // Structs can live both in the global scope and within inner scopes however 
    // structs defined within inner scopes are only accessible within that scope
    struct PlayerSession {
        user: User,
        session_id: String,
        is_active: bool,
        score: u32, 
    }
    // It is possible to store references to structs to data owned by something else
    // This requires the use of lifetimes to ensure the reference is valid for the duration of its use
    struct PlayerSessionRef<'a> {
        user: &'a User,
        session_id: &'a str, // &'a str - string slice with a lifetime 'a, references a string owned elsewhere
        is_active: bool,
        score: f64, 
    }

    // This is an immutable instance of the User struct
    let user = User {
        first_name: String::from("Alice"), // String::from creates a heap-allocated string
        middle_initial: 'B',
        last_name: String::from("Smith"),
        age: 30,
        email: String::from("alice@example.com"),
    };

    // This is a mutable instance of the User struct
    let mut user2 = User {
        first_name: String::from("Bob"),
        middle_initial: 'C',
        last_name: String::from("Johnson"),
        age: 25,
        email: String::from("bob@example.com"),
    };
    user2.age = 26; // Modifying the age of the mutable user2 instance


    let session = PlayerSession {
        user: user,
        session_id: String::from("session123"),
        is_active: true,
        score: 100,
    };

    println!("Player: {} {} {}", session.user.first_name, session.user.middle_initial, session.user.last_name);
    println!("Score: {}", session.score);

    println!("Player2: {} {} {}", user2.first_name, user2.middle_initial, user2.last_name);
    println!("Age: {}", user2.age);

}
