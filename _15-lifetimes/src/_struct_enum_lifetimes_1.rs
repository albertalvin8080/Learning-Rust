#[derive(Debug)]
struct NoCopyType;

#[derive(Debug)]
struct Borrower<'a> {
    x: &'a i32,
    y: &'a NoCopyType,
}

#[derive(Debug)]
enum EnumBorrower<'a> {
    Ref(&'a i32),
    Own(i32),
}

pub fn run() {
    let borrower: Borrower<'_>;
    let n: i32 = 43;

    {
        let nct = NoCopyType;
        borrower = Borrower {x: &n, y: &nct};

        println!("{:?}", borrower); // OK
    }
    // println!("{:?}", borrower); // ERROR: nct must outlive borrower.

    let eb: EnumBorrower<'_> = EnumBorrower::Ref(&433);
    println!("OK");
}

pub fn run2() {
    let borrower: Borrower<'_>;
    let n: i32 = 43;

    {
        borrower = Borrower {x: &n, y: &NoCopyType};
        println!("{:?}", borrower); // OK
    }
    println!("{:?}", borrower); // OK

    println!("OK");
}
