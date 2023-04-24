use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 6, 69];
    // let numbers: [i32; 5] = [1, 2, 3, 6];  // This throws arrow because array element count needs to be exact

    // Re-assign value
    numbers[2] = 420;
    println!("{:?}", numbers);

    // Get single val
    println!("Single value: {}", numbers[0]);

    // Get array length
    println!("Array length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[1..4];
    println!("Slice: {:?}", slice);
}