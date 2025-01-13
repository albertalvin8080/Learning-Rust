pub fn run() {
    {
        let author = String::from("Franz");

        // If we move it, we are no longer able to borrow by reference.
        // let print = move || println!("{}", author);
        let print = || println!("{}", author);

        print();
        print();

        let author_ref = &author;
        println!("{}", author);
    }
    {
        let mut n = 0;

        let mut add = || {
            n += 1;
            println!("{}", n);
        };

        add();
        add();
        add();

        let reborrow = &n;
        println!("{}", reborrow);
        // ERR: We cannot use the closure from now on because we created a immutable reference above.
        // add();

        let mut_reborrow = &mut n;
        // n += 1; // ERR: Already mutable borrowed
        // println!("{}", reborrow); // ERR: Cannot use it after mut_reborrow
        *mut_reborrow += 1; // OK

        assert_eq!(4, *mut_reborrow);
    }
    {
        let movable = Box::new(3);
        let consume = || {
            println!("{}", movable);
            // take(movable); // We can only call this closure once.
            take(movable.clone()); // We can call it as much as we want.
        };

        consume();
        consume();
    }
    {
        let s = String::from("Franz");

        let close = |x| x;

        close(s);
        // close(34); // ERR: Already inferred as String.
        close(34.to_string()); 
    }
}

fn take<T>(_: T) {}
