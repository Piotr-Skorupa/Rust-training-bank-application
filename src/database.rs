use std::str::FromStr;
use rust_decimal::Decimal;
use sqlite::{self, State};
use crate::bank::account::Account;


const CREATE_TABLE_ACCOUNTS: &str = "CREATE TABLE IF NOT EXISTS accounts (name TEXT UNIQUE, saldo DECIMAL);";

pub trait Database {
    fn connect(&mut self) -> bool;
    fn create_account(&mut self, name: String) -> bool;
    fn save_saldo(&mut self, name: String, saldo: Decimal) -> bool;
    fn read_saldo(&mut self, name: String) -> Option<Decimal>;
    fn get_account(&mut self, name: String) -> Option<Account>;
}


pub struct SQLiteDatabase {
}

impl SQLiteDatabase {
    pub fn new() -> SQLiteDatabase {
        SQLiteDatabase { }
    }
}

impl Database for SQLiteDatabase {
    fn connect(&mut self) -> bool {
        let connection: sqlite::Connection = sqlite::open("data.sqlite").unwrap();

        match connection.execute(CREATE_TABLE_ACCOUNTS, ) {
            Ok(_n) => { println!("Database connection success!"); return true },
            Err(_er) => { eprintln!("SQL error: {}", _er); return false }
        }
    }

    fn create_account(&mut self, name: String) -> bool {
        let connection: sqlite::Connection = sqlite::open("data.sqlite").unwrap();

        let statement: String = 
            "INSERT INTO accounts (name, saldo) VALUES ('".to_owned() + &name +
            "', 0.0);";

        println!("{}", statement);
        match connection.execute(statement, ) {
            Ok(_n) => { true },
            Err(_er) => { eprintln!("SQL error: {}", _er); false }
        }
    }

    fn save_saldo(&mut self, name: String, saldo: Decimal) -> bool {
        let connection: sqlite::Connection = sqlite::open("data.sqlite").unwrap();

        let statement: String = 
            "UPDATE accounts SET saldo=".to_owned() + &saldo.to_string() + " WHERE name='"
            + &name + "';";

        match connection.execute(statement, ) {
            Ok(_n) => { true },
            Err(_er) => { eprintln!("SQL error: {}", _er); false }
        }
    }

    fn read_saldo(&mut self, name: String) -> Option<Decimal> {
        let connection: sqlite::Connection = sqlite::open("data.sqlite").unwrap();

        let statement: String = "SELECT saldo FROM accounts".to_owned() + " WHERE name='" + &name + "';";

        let matcher = match connection.prepare(statement, ) {
            Ok(mut result) => {
                let mut saldo: String = "".to_string();
                while let State::Row = result.next().unwrap() {
                    saldo = result.read::<String>(0).unwrap();
                }
                match Decimal::from_str(saldo.as_str()) {
                    Ok(_saldo) => { Some(_saldo) },
                    Err(_er) => { None }
                }
             },
            Err(_er) => { eprintln!("SQL error: {}", _er); None }
        }; matcher
    }

    fn get_account(&mut self, name: String) -> Option<Account> {
        let connection: sqlite::Connection = sqlite::open("data.sqlite").unwrap();

        let statement: String = "SELECT * FROM accounts".to_owned() + " WHERE name='" + &name + "';";

        let matcher = match connection.prepare(statement, ) {
            Ok(mut result) => {
                let mut account = Account::new(name);
                let mut saldo: String = "".to_string();
                while let State::Row = result.next().unwrap() {
                    saldo = result.read::<String>(1).unwrap();
                }
                match Decimal::from_str(saldo.as_str()) {
                    Ok(_n) => { account.add_money(_n); Some(account) },
                    Err(_er) => { None }
                }
             },
            Err(_er) => { eprintln!("SQL error: {}", _er); None }
        }; matcher
    }
}
