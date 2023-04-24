enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right,
    Jump
}

fn move_avatar(m: Movement) {
    // Perform action depending on info
    match m {
        Movement::Up => println!("Player looks up!"),
        Movement::Down => println!("Player looks down!"),
        Movement::Left => println!("Player looks left!"),
        Movement::Right => println!("Player looks right!"),
        Movement::Jump => println!("Player jumps!")
    }
}

pub fn run() {
    let avatar1 = Movement::Up;
    let avatar2 = Movement::Down;
    let avatar3 = Movement::Left;
    let avatar4 = Movement::Right;
    let avatar5 = Movement::Jump;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
    move_avatar(avatar5);
}