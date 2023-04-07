// Primitive str = Immutable fixed length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    let mut hello: String = String::from("Hello ");

    // Get length
    println!("Length: {}", hello.len());

    // Push char
    hello.push('B');

    // Push string
    hello.push_str("ambang");

    println!("{}", hello);

    // Capacity in bytes
    println!("Capacity: {}", hello.is_empty());

    // Contains
    println!("Contains 'Bambang' {}", hello.contains("Bambang"));

    println!("{}", hello);
    
    // Replace
    println!("Replace: {}", hello.replace("Bambang", "Jumaidah"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(6);
    s.push('a');
    s.push('💀');

    // Assertion testing
    // assert_eq!(2, s.len());  // This will panic
    assert_eq!(6, s.capacity());

    println!("{}", s);
}
