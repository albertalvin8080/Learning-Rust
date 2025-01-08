#![allow(warnings)]

use std::collections::HashMap;
fn main() {
    // let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3, 4];
    v.push(99);
    println!("{v:?}");
    println!("{}", v[4]);

    let mut name = String::from("Helmuth");
    name.push(' ');
    name.push_str("Voss");
    println!("{name}");

    // let mut map: HashMap<i32, String> = HashMap::new();
    let mut map: HashMap<i32, &str> = HashMap::new(); // Also OK
    map.insert(1, "Summa Blasphemia");
    map.insert(2, "Detestatio Sacrorum");
    // println!("{map:?}");

    for (key, value) in map {
        println!("{key}: {value}");
    }
}
