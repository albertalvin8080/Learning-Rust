use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Author {
    name: String,
    age: u8,
}

impl Author {
    fn new(name: String, age: u8) -> Author {
        Author { name, age }
    }
}

pub fn run() {
    let mut h1: HashMap<&str, u8> = HashMap::new();
    h1.insert("Einar", 43);

    h1.entry("Ragnar").or_insert(100); // Inserts only if key doesnt exist
    h1.entry("Einar").or_insert_with(generate_val);

    assert_eq!(h1.get("Einar"), Some(&43));
    assert_eq!(h1.get("Ragnar"), Some(&100));

    let arr = [
        (Author::new("Franz".to_string(), 43), "Bonaparta"),
        (Author::new("Helmuth".to_string(), 29), "Voss"),
        (Author::new("Klaus".to_string(), 35), "Poppe"),
    ];

    let h2: HashMap<Author, &str> = HashMap::from(arr);
    for (k, v) in &h2 {
        println!("K -> {:?}; V -> {}", k, v);
    }
}

fn generate_val() -> u8 {
    50
}
