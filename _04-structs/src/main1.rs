#![allow(warnings)]

// self reference mutability
fn main() {
    let mut acc = BankAccount {
        owner: String::from("Franz Bonaparta"),
        balance: 100.0,
    };

    acc.view_balance();
    acc.withdrawn(30.1);
    acc.view_balance();
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn view_balance(&self) {
        println!("Balance: {:.2}", self.balance);
    }
    fn withdrawn(&mut self, quantity: f64) {
        self.balance -= quantity;
        println!("Withdrawn: {:.2}", quantity);
    }
}
