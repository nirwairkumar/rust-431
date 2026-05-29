// Demonstration on one mutable reference or may immutable references
fn main(){
    let mut account: BankAccount = BankAccount{
        owner: "Rubi".to_string(),
        balance:150.55,
    };
    // Immutable borrow to check the balance
    account.check_balance();   //>> Account owned by Rubi has a balance of 150.55

    // Mutable borrow to withdraw
    account.withdraw(50.5);   //>> Withdrawing 50.5 from account owned by Rubi
    account.check_balance();  //>> Account owned by Rubi has a balance of 100.05000000000001
}

struct BankAccount{
    owner: String,
    balance: f64,
}

impl BankAccount{
    fn withdraw(&mut self, amount:f64){
        println!("Withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self){
        println!("Account owned by {} has a balance of {}", self.owner, self.balance);
    }
}