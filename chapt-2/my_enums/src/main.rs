enum AccountStatus {
    Active,
    Suspended { reason: String,},
    Closed,
}

impl AccountStatus {
    fn is_unsafe(&self) -> bool {
        match self {
            AccountStatus::Active => false,
            AccountStatus::Suspended { .. } => true,
            AccountStatus::Closed => true,
        }
    }
}

fn main() {
    let status = AccountStatus::Suspended {
        reason: String::from("Payment overdue"),
    };
    println!("Is unsafe: {}", status.is_unsafe());
}