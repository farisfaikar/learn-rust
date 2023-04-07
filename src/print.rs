pub fn run() {
    // Print to console
    println!("Hello from the other side (print.rs)");

    // Basic formatting
    println!("Number: {}", 1);
    println!("{} is from {}", "Chad", "Ohio");

    // Positional arguments
    println!(
        "{0} is from {1} and {0} likes to {2}.",
        "Chad", "Ohio", "fuck"
    );

    // Named arguments
    println!(
        "{name} likes to {lure} children",
        name = "John",
        lure = "lure"
    );

    // Placeholder traits
    println!("Binary {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hewwo"));

    // Basic math
    println!("69 + 420 = {}", 69 + 420);
}
