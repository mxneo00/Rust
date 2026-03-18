mod loops;
mod for_loops;
mod if_statements;
mod while_loops;

fn main() {
    println!("Demonstrating loops:");
    println!("-------------------------------");
    loops::loops();
    println!("-------------------------------");
    println!("Demonstrating for loops:");
    println!("-------------------------------");
    for_loops::for_loops();
    println!("-------------------------------");
    println!("Demonstrating if statements:");
    println!("-------------------------------");
    if_statements::if_statements();
    println!("-------------------------------");
    println!("Demonstrating while loops:");
    println!("-------------------------------");
    while_loops::while_loops();
    println!("-------------------------------");

}
