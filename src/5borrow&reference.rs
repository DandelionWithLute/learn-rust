// Reference ?
fn main() {
    let mut account: BankAccount = BankAccount {
        owner: String::from("Alice"),
        balance: 150.55,
    };

    account.check_balance();

    account.widthdraw(103.45);

    account.check_balance();
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn widthdraw(&mut self, amount: f64) {
        println!(
            "Widthdrawing {} from account owned by {}",
            amount, self.owner
        );
        self.balance -= amount
    }

    fn check_balance(&self) {
        println!(
            "{}'s account has the balance of {}",
            self.owner, self.balance
        )
    }
}

// Another Teaching Example (Borrowing)
fn borrow() {
    let mut _x: i32 = 5;
    let _r: &mut i32 = &mut _x;

    // reference cannot be written
    // [* is essential] cannot mutate immutable variable `_r`rust-analyzerE0384
    *_r += 1;
    *_r -= 3;

    println!("Value of _x: {}", _x);

    // not allowed below because
    // mutable borrowed can't be immutable borrowed
    // cannot borrow `_x` as immutable because it is also borrowed as mutable

    // println!("Value of _r: {}", _r);
}
