use std::ptr::eq;

use rust_decimal::{Decimal, prelude::FromPrimitive};


#[derive(Debug, Clone)]
pub struct Account {
    name_: String,
    money_: Decimal
}

impl Account {
    pub fn new(name: String) -> Account {
        Account { name_: name, money_: Decimal::new(00, 1) }
    }

    pub fn name(&self) -> String {
        self.name_.clone()
    }

    pub fn get_money(&self) -> Decimal {
        self.money_
    }

    pub fn add_money(&mut self, money: Decimal) -> bool {
        if money > Decimal::from_i32(0).unwrap() {
            self.money_ += money;            
            return true;
        }

        false
    }

    pub fn subtract_money(&mut self, money: Decimal) -> bool {
        if (self.money_ - money) >= Decimal::from_i32(0).unwrap() {
            self.money_ -= money;
            return true;
        }

        false
    }
}

impl PartialEq for Account {
    fn eq(&self, other: &Account) -> bool {
        return self.name_ == other.name_;
    }

    fn ne(&self, other: &Account) -> bool { return !eq(self, other); }
}
