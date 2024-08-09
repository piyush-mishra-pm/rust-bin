use std::fmt::format;

#[derive(Debug)]
struct Account{
    id:u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id:u32, holder: String) -> Self {
        Account {id, holder, balance:0 }
    }

    fn deposit(&mut self, amount:i32) -> i32 {
        self.balance+=amount;
        self.balance

    }

    fn withdraw(&mut self, amount:i32) ->i32 {
        self.balance-=amount;
        self.balance
    }

    fn summary(&self) -> String {
        format!("{} has a balance {}",  self.holder, self.balance)
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank {accounts: vec![] }
    }

    fn add_account(&mut self, account: Account){
        self.accounts.push(account);
    }

    fn total_balance(&self)->i32{
        self.accounts.iter().map(|account| account.balance).sum()
    }

    fn summary(&self) -> Vec<String>{
        self.accounts.iter().map(|acc| acc.summary()).collect::<Vec<String>>()
    }
}

fn main() {
    let mut bank = Bank::new();
    let mut account_1 = Account::new(1, String::from("Piyush"));
    account_1.deposit(10);
    account_1.withdraw(15);
    account_1.deposit(20);
    println!("{}",account_1.summary());

   bank.add_account(account_1);
   
   let mut account_2 = Account::new(2, String::from("Ayush"));
   account_2.deposit(100);

   bank.add_account(account_2);

   println!("{:#?}", bank.summary());
   println!("{:#?}", bank.total_balance());
}