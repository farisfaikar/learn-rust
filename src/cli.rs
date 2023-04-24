use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command: String = args[1].clone();
    let name: &str = "Alex";
    let status: u8 = 90;

    if command == "hello" {
        println!("Hello, {name}!");
    } else if command == "status" {
        println!("Status: {status}%", );
    } else {
        println!("That is not a valid command you shithead");
    }

    println!("Command: {}", command);
}