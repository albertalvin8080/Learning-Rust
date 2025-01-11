trait MyTrait: std::fmt::Debug {
    // fn f(&self) -> &dyn MyTrait;
    fn f(&self) -> Box<dyn MyTrait>;
}

impl MyTrait for i32 {
    fn f(&self) -> Box<dyn MyTrait> {
        Box::new(*self)
    }
}

impl MyTrait for String {
    fn f(&self) -> Box<dyn MyTrait> {
        Box::new(self.clone())
    }
}

fn call_f(t: &impl MyTrait) -> Box<dyn MyTrait> {
    t.f()
}

// Object Safe Traits
pub fn run () {
    let t1 = 100_i32;
    let t2 = "Franz Bonaparta".to_string();

    call_f(&t1);
    call_f(&t2);
    println!("{:?}", call_f(&t1));
}