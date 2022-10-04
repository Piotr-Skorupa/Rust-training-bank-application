use self::account::Account;
use crate::database::Db;
use rust_decimal::Decimal;

pub mod account;


pub trait Bank {
    fn new() -> Self;
    fn create_account(&mut self, name: &String) -> bool;
    fn print_account_names(&self);
    fn set_active_account(&mut self, name: &String) -> bool;
    fn saldo(&self);
    fn deposit(&mut self, money: Decimal);
    fn withdraw(&mut self, money: Decimal) -> bool;
    fn transfer_to(&mut self, target_name: &String, money: Decimal) -> bool;
}

pub struct PioBank {
    accounts_: Vec<Account>,
    active_account_index_: Option<usize>,
    db_: Db
}

impl Bank for PioBank {
    fn new() -> PioBank {
        let mut self_ = PioBank { accounts_: Vec::<Account>::new(), active_account_index_: None, db_: Db::new() };
        self_.db_.connect();
        return self_;
    }

    fn create_account(&mut self,  name: &String) -> bool {
        let account: Account = Account::new(name.to_string());

        if self.accounts_.contains(&account) {
            return false;
        }

        if !self.db_.create_account(name.clone()) {
            return false;
        }

        println!("Account {} has been created!", name);
        self.accounts_.push(account);
        return true;
    }

    fn print_account_names(&self) {
        println!("Accounts: {:?}", self.accounts_);
    }

    fn set_active_account(&mut self, name: &String) -> bool {
        let account: Account;
        match self.db_.get_account(name.clone()) {
            Some(acc) => { account = acc },
            None => { return false; }
        }

        if !self.accounts_.contains(&account) {
            self.accounts_.push(account.clone());
        }
        
        let index = self.accounts_.iter().position(|e: &Account| e == &account).unwrap();
        self.active_account_index_ = Some(index);

        return true;
    }

    fn saldo(&self) {
        if self.active_account_index_.is_none() {
            eprintln!("No active account has been choosen!");
        }

        println!("SALDO: {} PLN", self.accounts_[self.active_account_index_.unwrap()].get_money());
        
    }

    fn deposit(&mut self, money: Decimal) {
        if self.active_account_index_.is_none() {
            eprintln!("No active account has been choosen!");
        }

        self.accounts_[self.active_account_index_.unwrap()].add_money(money);
        self.db_.save_saldo(self.accounts_[self.active_account_index_.unwrap()].name(),
            self.accounts_[self.active_account_index_.unwrap()].get_money());
    }

    fn withdraw(&mut self, money: Decimal) -> bool {
        if self.active_account_index_.is_none() {
            eprintln!("No active account has been choosen!");
        }

        let account: &mut Account = &mut self.accounts_[self.active_account_index_.unwrap()];
        if account.get_money() < money {
            eprintln!("You don't have enough money on your account! Please inupt different value.");
            return false;
        }

        return account.subtract_money(money);
    }

    fn transfer_to(&mut self, target_name: &String, money: Decimal) -> bool {
        if self.active_account_index_.is_none() {
            eprintln!("No active account has been choosen!");
            return false;
        }

        let target_account_pattern: Account = Account::new(target_name.to_string());

        if !self.accounts_.contains(&target_account_pattern) {
            eprintln!("Can not find target account!");
            return false;
        }
        
        let target_account_index = self.accounts_.iter().position(|e: &Account| e == &target_account_pattern).unwrap();

        let mut result: bool = self.accounts_[self.active_account_index_.unwrap()].subtract_money(money);
        if !result {
            eprintln!("You don't have enough money on your account! Please inupt different value.");
            return result;
        }

        result = self.accounts_[target_account_index].add_money(money);
        if !result {
            eprintln!("Target account rejected your transfer! Money is returning to you");
            self.accounts_[self.active_account_index_.unwrap()].add_money(money);
            return result;
        }

        return result;
    }

}