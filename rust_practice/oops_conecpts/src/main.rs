// Define a trait representing a generic Animal
trait Animal {
    fn sound(&self) -> String;
}

// Implement the Animal trait for a Dog struct
struct Dog {
    name: String,
}

impl Animal for Dog {
    fn sound(&self) -> String {
        format!("{} says woof!", self.name)
    }
}

// Implement the Animal trait for a Cat struct
struct Cat {
    name: String,
}

impl Animal for Cat {
    fn sound(&self) -> String {
        format!("{} says meow!", self.name)
    }
}

// Define a function to print the sound of an animal
fn print_sound(animal: &dyn Animal) {
    println!("{}", animal.sound());
}

fn main() {
    let dog = Dog {
        name: String::from("Buddy"),
    };
    let cat = Cat {
        name: String::from("Whiskers"),
    };

    // Polymorphism: We can treat Dog and Cat objects uniformly through the Animal trait
    print_sound(&dog);
    print_sound(&cat);
}
