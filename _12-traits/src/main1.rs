#![allow(warnings)]

struct Sheep;
struct Human;
trait Animal {
    fn some_default_method(&self) {
        println!("Im a default method which can be overriden.");
    }
    fn make_sound(&self) -> String;
}

impl Animal for Human {
    fn make_sound(&self) -> String {
        "unlimited blade works".to_string()
    }
}

impl Animal for Sheep {
    fn make_sound(&self) -> String {
        "meheehe".to_string()
    }
}

fn emmit_noise(animal: &impl Animal) {
    println!("{}", animal.make_sound());
}

// Trait bound
fn emmit_noise_trait_bound<T: Animal>(animal: &T) {
    println!("{}", animal.make_sound());
}

fn get_random_animal(n: u8) -> Box<dyn Animal> {
    if n >= 5 {
        Box::new(Human)
    } else {
        Box::new(Sheep)
    }
}

fn emmit_noise_dyn(animal: &dyn Animal) {
    println!("{}", animal.make_sound());
}

fn main() {
    let sheep = Sheep;
    let human = Human;

    emmit_noise(&sheep);
    emmit_noise_trait_bound(&human);

    let random_number = 5_u8;
    let animal: Box<dyn Animal> = get_random_animal(random_number);
    emmit_noise_dyn(&*animal);
}
