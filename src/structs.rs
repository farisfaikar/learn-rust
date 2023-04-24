struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple struct
struct Color2(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
    age: u8
}

impl Person {
    // Construct person
    fn new(first: &str, last: &str, age: u8) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
            age: age
        }
    }

    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c: Color = Color {
        red: 255,
        green: 0,
        blue: 128
    };

    c.green = 100;

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut c2: Color2 = Color2(122, 102, 42);

    c2.0 = 100;

    println!("Color2: {} {} {}", c2.0, c2.1, c2.2);

    let mut p: Person  = Person::new("Faris", "Faikar", 21);

    p.age = 20;

    println!("{}", p.full_name());

    let mut p2 = Person::new("Mary", "Poppins", 50);
    println!("Person {}", p2.full_name());
    p2.set_last_name("Jane");
    println!("Person {}", p2.full_name());
    println!("Person Tuple {:?}", (p2.to_tuple()));
}