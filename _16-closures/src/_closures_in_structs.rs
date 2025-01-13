pub fn run() {
    let mut cacher1 = Cacher::new(|x: i32| x + 5);
    println!("{}", cacher1.value(35));
    println!("{}", cacher1.value(999)); // Prints the same result due to caching.

    let mut cacher2 = Cacher::new(
        |x: &str| {
            let s: String = String::from("Helmuth") + x;
            // s.as_str()            // ERR: cannot return value referencing local variable `s`
            // s.to_owned()          // ERR: expected `&str`, found `String`
            // s.to_owned().as_str() // ERR: cannot return value referencing temporary value
            let _s: &'static mut String = Box::leak(Box::new(s.to_owned())); // OK
            _s // NOTE: &String is inferred as &str anyway
        }
    );
    println!("{}", cacher2.value(" Voss"));
    println!("{}", cacher2.value(" Farobek")); // Prints the same result due to caching.
}

struct Cacher<F, V>
where
    F: Fn(V) -> V,
    V: Copy,
{
    query: F,
    value: Option<V>,
}

// Needs again the Trait Bounds
impl<F, V> Cacher<F, V>
where
    F: Fn(V) -> V,
    V: Copy,
{
    fn new(query: F) -> Self {
        Cacher { query, value: None }
    }
    fn value(&mut self, arg: V) -> V {
        match self.value {
            Some(v) => v,
            None => {
                let result: V = (self.query)(arg);
                self.value = Some(result);
                result
            }
        }
    }
}
