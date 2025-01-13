pub fn run() {
    {
        let v = vec![1, 2, 3];
        // v immutably borrowed
        f_fn(|x| x == v.len());
    }
    {
        let mut s = String::new();
        let closure = |_str| s.push_str(_str);
        exec(closure);
        println!("{}", s);
    }
    {
        let s = String::from("Jakub");
        let c = move || println!("{}", s);
        exec_fn(c);
    }
}

fn exec_fn<F>(fun: F)
where
    F: Fn(),
{
    fun();
}

fn f_fn<F>(fun: F)
where
    F: Fn(usize) -> bool,
{
    println!("{}", fun(3));
    println!("{}", fun(4));
}

fn exec<'a, F: FnMut(&'a str) -> ()>(mut fun: F) {
    fun("Helmuth");
    fun("-");
    fun("Voss");
}
