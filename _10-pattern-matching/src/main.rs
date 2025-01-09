#![allow(warnings)]

enum MyEnum {
    Color(u8, u8, u8),
    Point { x: u32, y: u32 },
}

fn main() {
    let p = MyEnum::Point { x: 3, y: 4 };
    if let MyEnum::Point { x, y } = p {
        println!("{x},{y}");
    }

    let c = MyEnum::Color(99, 110, 120);
    if let MyEnum::Color(r @ (100 | 99), g @ (109 | 0..=110), b @ 120) = c {
        println!("{r},{g},{b}");
    }

    match p {
        MyEnum::Point {
            x: 0..=100,
            y: newY @ (4),
        } => {
            // println!("{x}, {newY}"); // W cant use x because its consumed in the matching.
            println!("MyEnum::Point {{ x: 0..=100, y: newY @ (4) }} => {newY}");
        }
        _ => {}
    }

    let m = 5;
    let matches_or_not = matches!(m, (0..=4 | 5));
    println!("matches: {matches_or_not}");
    
    let matches_or_not = matches!(Some(5), (Some(5)));  // true
    let matches_or_not = matches!(Some(5), (Some(n @ (0..=4)) | Some(n @ 5)));  // true
    let matches_or_not = matches!(Some(5), (Some(n @ (0..=4)) | Some(n @ -1))); // false
    println!("matches: {matches_or_not}");
    
    println!("Vec:");
    let v: Vec<char> = vec!['A', 'z', 'J', '@'];
    for ref c in v {
        let matches_or_not = matches!(c, ('A'..='Z' | 'a'..='z' | '@')); // true for '@' char
        let matches_or_not = matches!(c, ('A'..='Z' | 'a'..='z')); // false for '@' char
        println!("- matches: {matches_or_not}");
    } 

    let _x = MyEnum::Point { x:3, y: 5 };
    let other_variable = false;

    match _x {
        // Match Guard
        MyEnum::Point { x: x @ 3, y: y @ 5 } if other_variable => {
            println!("{x}, {y}");
        }
        _ => println!("No match was found.")
    }
}
