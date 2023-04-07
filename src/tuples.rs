// Tuples group together values of different types
// Max 12 elements

pub fn run() {
    let mandalorian: (&str, &str, i8) = ("Din Djarin", "Aq Vetina", 36);

    println!("{} is from {} and is {}", mandalorian.0, mandalorian.1, mandalorian.2);
}