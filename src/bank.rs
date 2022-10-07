use self::account::Account;
use crate::database::{Database, SQLiteDatabase};
use rust_decimal::Decimal;

pub mod account;


pub trait Bank {
    fn new() -> Self;
    fn create_account(&mut self, name: &String) -> bool;
    fn set_active_account(&mut self, name: &String) -> bool;
    fn saldo(&mut self);
    fn deposit(&mut self, money: Decimal);
    fn withdraw(&mut self, money: Decimal) -> bool;
    fn transfer_to(&mut self, target_name: &String, money: Decimal) -> bool;
}

pub struct PioBank {
    active_account_: Option<Account>,
    db_: Box<dyn Database>
}

impl Bank for PioBank {
    fn new() -> PioBank {
        let mut self_ = PioBank { active_account_: None, db_: Box::new(SQLiteDatabase::new()) };
        self_.db_.connect();
        return self_;
    }

    fn create_account(&mut self,  name: &String) -> bool {
        let account: Account = Account::new(name.to_string());

        if !self.db_.create_account(name.clone()) {
            eprintln!("Account {} alredy exists!", name);
            return false;
        }

        println!("Account {} has been created!", name);
        self.active_account_ = Some(account);
        return true;
    }

    fn set_active_account(&mut self, name: &String) -> bool {
        let account: Account;
        match self.db_.get_account(name.clone()) {
            Some(acc) => {
                account = acc
            },
            None => {
                eprintln!("Account {} doesn't exist!", name);
                return false; 
            }
        }

        self.active_account_ = Some(account);
        return true;
    }

    fn saldo(&mut self) {
        if self.active_account_.is_none() {
            eprintln!("No active account has been choosen!");
        }

        let active_account: &mut Account = self.active_account_.as_mut().unwrap();
        match self.db_.read_saldo(active_account.name()) {
            Some(money) => { active_account.set_money(money) },
            None => {}
        }

        println!("SALDO: {} PLN", active_account.get_money());
    }

    fn deposit(&mut self, money: Decimal) {
        if self.active_account_.is_none() {
            eprintln!("No active account has been choosen!");
        }

        let active_account: &mut Account = self.active_account_.as_mut().unwrap();

        active_account.add_money(money);
        self.db_.save_saldo(active_account.name(), active_account.get_money());
    }

    fn withdraw(&mut self, money: Decimal) -> bool {
        if self.active_account_.is_none() {
            eprintln!("No active account has been choosen!");
        }

        let active_account: &mut Account = self.active_account_.as_mut().unwrap();

        if active_account.get_money() < money {
            eprintln!("You don't have enough money on your account! Please inupt different value.");
            return false;
        }

        if !active_account.subtract_money(money) {
            return false;
        }

        return self.db_.save_saldo(active_account.name(), active_account.get_money());
    }

    fn transfer_to(&mut self, target_name: &String, money: Decimal) -> bool {
        if self.active_account_.is_none() {
            eprintln!("No active account has been choosen!");
            return false;
        }

        let active_account: &mut Account = self.active_account_.as_mut().unwrap();
        let mut target_account: Account;
        match self.db_.get_account(target_name.to_string()) {
            Some(acc) => {
                target_account = acc;
            },
            None => {
                eprintln!("Target account {} doesn't exist! Can not transfer money", target_name);
                return false;
            }
        }

        if !active_account.subtract_money(money) {
            eprintln!("You don't have enough money on your account! Please inupt different value.");
            return false;
        }

        if !target_account.add_money(money) {
            eprintln!("Target account rejected your transfer! Money is returning to you");
            active_account.add_money(money);
            return self.db_.save_saldo(active_account.name(), active_account.get_money());
        }

        self.db_.save_saldo(active_account.name(), active_account.get_money());
        return self.db_.save_saldo(target_name.to_owned(), target_account.get_money());
    }

}