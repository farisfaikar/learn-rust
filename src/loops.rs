pub fn run() {
    let mut count: i32 = 0;

    // Infinite loop
    loop {
        count += 1;
        println!("Numbers: {}", count);

        if count >= 20 {
            println!("Count reached {}. Terminating", count);
            break;
        }
    }

    // While loop (FizzBuzz)
    while count <= 100 {
        if count % 15 == 0 {
            println!("FizzBuzz");
        } else if count % 3 == 0 {
            println!("Fizz");
        } else if count % 5 == 0 {
            println!("Bizz");
        } else {
            println!("{}", count);
        }
        // Increment
        count += 1;
    }

    for x in 0..100 {
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0 {
            println!("Bizz");
        } else {
            println!("{}", x);
        }
    }
}