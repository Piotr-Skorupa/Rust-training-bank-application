use std::ptr::null;
use rust_decimal::Decimal;
use sqlite::Connection;

pub struct Db {
}

impl Db {
    pub fn new() -> Db {
        Db { }
    }

    pub fn connect(&mut self) -> bool {
        let connection = sqlite::open("data.sqlite").unwrap();

        let statement: String = "CREATE TABLE IF NOT EXISTS accounts (name TEXT UNIQUE, saldo DECIMAL);".to_string();
        match connection.execute(statement, ) {
            Ok(_n) => { println!("Database connection success!"); return true },
            Err(_er) => { eprintln!("SQL error: {}", _er); return false }
        }
    }

    pub fn create_account(&mut self, name: String) -> bool{
        let connection = sqlite::open("data.sqlite").unwrap();

        let statement: String = 
            "INSERT INTO accounts (name, saldo) VALUES ('".to_owned() + &name +
            "', 0.0);";

        println!("{}", statement);
        match connection.execute(statement, ) {
            Ok(_n) => { true },
            Err(_er) => { eprintln!("SQL error: {}", _er); false }
        }
    }

    pub fn save_saldo(&mut self, name: String, saldo: Decimal) {
        let connection = sqlite::open("data.sqlite").unwrap();

        let statement: String = 
            "UPDATE accounts saldo=".to_owned() + &saldo.to_string() + " WHERE name="
            + &name + ";";

        match connection.execute(statement, ) {
            Ok(_n) => { },
            Err(_er) => { eprintln!("SQL error: {}", _er) }
        }
    }
    
}
