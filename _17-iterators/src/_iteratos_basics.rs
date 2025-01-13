use std::{collections::HashMap, iter};

pub fn run() {
    {
        let v = vec![1, 2];
        let mut it = v.into_iter();
        println!("{:?}", it.next());
        println!("{:?}", it.next());
        println!("{:?}", it.next());
        // println!("{}", v); // ERR: into_iter() transfers ownership
    }
    {
        let mut v = vec![1, 2, 3];

        // Mutable reference iterator
        if let Some(n) = v.iter_mut().next() {
            *n = 0;
        }

        assert_eq!(v, vec![0, 2, 3]);
    }
    {
        let mut f = Fibonacci::new();
        print!("{:?} ", f.next());
        print!("{:?} ", f.next());
        print!("{:?} ", f.next());
        print!("{:?} ", f.next());
        println!("{:?} ", f.next());
    }
    {
        let v = vec![1, 2, 3];
        // map() takes ownership of the values, but i32 implements Copy trait, so v is still usable.
        let v2: Vec<_> = v.iter().map(|x: &i32| x + 10).collect();

        println!("{:?}", v);
        println!("{:?}", v2);
    }
    {
        let names = ["sunface", "sunfei"];
        let ages = [18, 18];

        let mut it = ages.into_iter();
        let folks: HashMap<&str, i32> =
            names.into_iter().map(|x| (x, it.next().unwrap())).collect();

        println!("{:?}", folks);
    }
    {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);

        // println!("{:?}, {:?}", v1, v1_iter); // v1_iter lost ownership of values when sum() was called.
        println!("{:?} = {}", v1, total);
    }
}

struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Fibonacci {
    fn new() -> Self {
        Self { curr: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let forward = self.curr + self.next;
        self.curr = self.next;
        self.next = forward;

        Some(self.curr)
    }
}
