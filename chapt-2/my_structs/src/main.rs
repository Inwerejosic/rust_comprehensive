// importing needed external crates(packages and modules)
use uuid::Uuid;
use std::fmt;

// Defining a struct named Account with fields for id, name, and balance
struct Account {
    id: Uuid,
    name: String,
    balance: f64,
}

// Implementing the Display trait for the Account struct to allow for formatted output
impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Account {{ id: {}, name: {}, balance: {} }}", 
        self.id, 
        self.name, 
        self.balance)
    }
}

// Implementing methods for the Account struct
impl Account {
    // Creating a new Account instance with a unique id, name, and balance
    fn new(name: String, balance: f64) -> Self {
        Account {
            id: Uuid::new_v4(),
            name,
            balance,
        }
    }

    // Depositing money into the account
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    // Withdrawing money from the account
    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if self.balance >= amount {
            self.balance -= amount;
            Ok(())
        } else {
            Err(String::from("Insufficient funds"))
        }
    }
}

// The main function demonstrates the usage of the Account struct and its methods
fn main() {
    let account = Account::new(String::from("John Doe"), 1000.0);
    println!("{:#}", account);

    let mut account = account;
    account.deposit(500.0);
    println!("After deposit: {}, {}", account.name, account.balance);

    let result = account.withdraw(100.0);
    match result {
        Ok(_) => println!("Withdrawal successful. New balance: {}, {}", account.name, account.balance),
        Err(e) => println!("Withdrawal failed: {}", e),
    }
}
