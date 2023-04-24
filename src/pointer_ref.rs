pub fn run() {
    // Primitize array
    let arr1: [i32; 3] = [1, 2, 3];
    let arr2: [i32; 3] = arr1;

    println!("Array values: {:?}", (arr1,));

    // Vector
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;

    println!("Vector values: {:?}", (&vec1, vec2))
}