pub fn run() {
    greeting("Hi, nice to meet you", "Faris");

    // Bind function values to variables
    let get_sum: i32 = add(5, 5);
    println!("Sum: {}", get_sum);

    // Closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2;
    println!("C Sum: {}", add_nums(3, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{}. My name is {}", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}