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
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank {accounts: vec![] }
    }
}

fn print_account(account: &mut Account) {
    println!("Adding balance to account:");
    account.balance+=10;
    println!("{:#?}", account);
}

fn add_account_to_bank(bank: &mut Bank, account: Account){
    println!("Adding account to bank:");
    bank.accounts.push(account);
}

fn main() {
    let mut bank = Bank::new();
    let mut account = Account::new(1, String::from("Piyush"));

    print_account(&mut account);
    add_account_to_bank(&mut bank, account);
    println!("{:#?}", bank);
}