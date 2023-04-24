pub fn run() {
    // let test: &str = "test";
    let age: u8 = 5;
    let check_id: bool = false;
    let knows_person_of_age: bool = true;

    // If/Else
    if age >= 21 || knows_person_of_age {
        println!("Barkeep: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Barkeep: Sorry you gotta buzz off.")
    } else {
        println!("Barkeep: Get the hell outta here, you're {} years old.", age);
    }

    // Shorthand if
    let is_of_age: bool = if age >= 21 { true } else { false };
    println!("Is Of Age: {}", is_of_age);
}