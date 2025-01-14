use std::{cell::RefCell, thread};

pub fn run() {
    let mut a = Node {
        value: RefCell::new(String::from("Franz Bonaparta")),
        adjacent: vec![],
    };

    let mut b = Node {
        value: RefCell::new(String::from("Helmuth Voss")),
        adjacent: vec![&a],
    };

    let mut c = Node {
        value: RefCell::new(String::from("Jakub Farobek")),
        adjacent: vec![&a, &b],
    };

    add_mark(&c);

    println!("{:#?}", c);
}

#[derive(Debug)]
struct Node<'a> {
    value: RefCell<String>,
    adjacent: Vec<&'a Node<'a>>,
}

fn add_mark(node: &Node) {
    let mut b = node.value.borrow_mut();
    b.push_str("!");

    // n: &&Node<'_>
    for n in node.adjacent.iter() {
        add_mark(*n);
    }
}
