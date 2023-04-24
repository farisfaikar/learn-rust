// Vectors (resizeable arrays)

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 6, 69];
    // let numbers: [i32; 5] = [1, 2, 3, 6];  // This throws arrow because array element count needs to be exact

    // Re-assign value
    numbers[2] = 420;
    println!("{:?}", numbers);

    // Add on to vector
    numbers.push(5);
    numbers.push(8008);

    // Pop off last value
    numbers.pop();

    // Get single val
    println!("Single value: {}", numbers[0]);

    // Get vector length
    println!("Vector length: {}", numbers.len());

    // Vectors are ... allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[1..4];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", numbers);

    
}