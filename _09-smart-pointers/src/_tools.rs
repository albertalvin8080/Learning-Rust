use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub fn run() {
    let mut franz = Rc::new(RefCell::new(Owner {
        name: "Franz Bonaparta".to_string(),
        tools: vec![],
    }));

    let tool1: Rc<Tool> = Rc::new(Tool {
        name: "tool1".to_string(),
        owner: Rc::clone(&franz),
    });

    let tool2 = Rc::new(Tool {
        name: "tool2".to_string(),
        owner: Rc::clone(&franz),
    });

    // Rc::downgrade is used to create a Weak<> smart pointer.
    franz.borrow_mut().tools.push(Rc::downgrade(&tool1));
    franz.borrow_mut().tools.push(Rc::downgrade(&tool2));

    println!("{:#?}", franz);
    println!("{:#?}", franz.borrow().tools[0].upgrade().unwrap().name);
}

#[derive(Debug)]
struct Owner {
    name: String,
    tools: Vec<Weak<Tool>>,
}

#[derive(Debug)]
struct Tool {
    name: String,
    owner: Rc<RefCell<Owner>>,
}
