pub fn run() {
    let m: MyType<'_> = MyType(&43);
    m.return_ref("some string literal");
    println!("OK");
}

struct MyType<'a>(&'a u8);

impl<'a> MyType<'a> {
    // Elision 3rd rule doesn't apply here (?)
    fn return_ref(&self, x: &'a str) -> &str {
        x
    }
}
