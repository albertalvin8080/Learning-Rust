use std::cell::Cell;

pub fn run() {
    let mut a = Node {
        value: Cell::new(10),
        adjacent: vec![],
    };

    let mut b = Node {
        value: Cell::new(56),
        adjacent: vec![&a],
    };

    let mut c = Node {
        value: Cell::new(32),
        adjacent: vec![&a, &b],
    };

    add_one(&c);

    println!("{:#?}", c);
}

#[derive(Debug)]
struct Node<'a> {
    value: Cell<i32>,
    adjacent: Vec<&'a Node<'a>>,
}

fn add_one(node: &Node) {
    let new_val = node.value.get() + 1;
    node.value.set(new_val);

    // n: &&Node<'_>
    for n in node.adjacent.iter() {
        add_one(*n);
    }
}
