/*
You can create a project using cargo:
- cargo new testproject (or `cargo init` inside the folder)
- cargo run
*/
fn main() {
    let c: char = 'b';
    println!("{}", c);

    let x: u32 = 100;
    println!("u32: {}", x);

    let y: i8 = -100;
    println!("i8: {}", y);

    // &str   -> slice string, its a reference. Borrowed. Immutable.
    // String -> owned, mutable.
    let a: &str = "Franz Bonaparta";
    println!("{}", a);

    let arr: [i16; 5] = [1, 2, -3, 4, 5];
    println!("{:?}", arr);

    let tuple: (String, u8, bool) = ("Helmuth".to_string(), 39, false);
    println!("{:?}", tuple);

    //Slices
    let myslice: &[i32] = &[34, 34, 20];
    println!("{:?}", myslice);

    // let other_slice: &[String] = &["Voss".to_string()]; // reference to a slice of strings.
    let other_slice: &[&String] = &[&"Voss".to_string()]; // reference to a slice of references to strings.
    // println!("{:?}", other_slice);

    println!("{:?}", other_slice);
}
