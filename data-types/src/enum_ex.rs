// Enums in Rust
// Enums allow you to define a type by enumerating its possible variants
// Each variant can optionally have data associated with it

pub fn enum_example() {
    enum IpAddr {
        // Rather than an enum variant being just a name, it can also have associated data
        // Each variant can have different types and amounts of associated data
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    match home {
        IpAddr::V4(a, b, c, d) => println!("IPv4 address: {}.{}.{}.{}", a, b, c, d),
        IpAddr::V6(addr) => println!("IPv6 address: {}", addr),
    }

    match loopback {
        IpAddr::V4(a, b, c, d) => println!("IPv4 address: {}.{}.{}.{}", a, b, c, d),
        IpAddr::V6(addr) => println!("IPv6 address: {}", addr),
    }

}