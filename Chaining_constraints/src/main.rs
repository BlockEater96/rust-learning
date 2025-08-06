struct BankAccount {
    balance: f64,
    owner: String,
}
impl BankAccount {
    fn new(owner: String, initial_balance: f64) -> Self {
        println!("Creating a new bank account for {}", owner);
        Self {
            balance: initial_balance,
            owner,
        }
    }
    fn change_owner(mut self, new_owner: String) -> Self {
        println!("Changing owner from {} to {}", self.owner, new_owner);
        self.owner = new_owner;
        self
    }
    fn deposit(&mut self, amount: i32) -> &mut Self {
        if amount > 0 {
            println!("Depositing {} into account of {}", amount, self.owner);
            self.balance += amount as f64;
        } else {
            println!("Deposit amount must be positive");
        }
        self
    }
    fn withdraw(&mut self, amount: i32) -> &mut Self {
        if amount > 0 && amount as f64 <= self.balance {
            println!("Withdrawing {} from account of {}", amount, self.owner);
            self.balance -= amount as f64;
        } else if amount <= 0 {
            println!("Withdrawal amount must be positive");
        } else {
            println!("Insufficient funds for withdrawal");
        }
        self
    }
    fn display_balance(&self) {
        println!("Current balance for {}: ${:.2}", self.owner, self.balance);
    }
    fn view_owner(&self) {
        println!("Account owner: {}", self.owner);
    }
}

fn main() {
    println!("=====Chaining constraints in Rust=====");
    let mut account = BankAccount::new("John Doe".to_string(), 1000.0);
    account.view_owner();
    account.display_balance();
    account.deposit(500);
    account.withdraw(200);
    account
        .change_owner("Jane Smith".to_string())
        .display_balance();
}
