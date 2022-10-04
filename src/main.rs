use std::{str, process::exit};
use rust_decimal::{Decimal, prelude::*};
use bank::{Bank, PioBank};

mod bank;
mod database;


static BANK_NAME: &str = "[************* PioBank *************]";
static EXIT_CHOICE: &str = "q";
const EXIT_CODE_SUCCESS: i32 = 0;
const EXIT_CODE_ABORT: i32 = 6;

fn abort_exit(message: &str) {
    eprintln!("{}", message);
    exit(EXIT_CODE_ABORT);
}

fn internal_loop(pio_bank: &mut PioBank) {
    let mut line: String = String::new();
    while line != String::from(EXIT_CHOICE) {
        line.clear();
        println!("1 - Check saldo");
        println!("2 - Deposit");
        println!("3 - Withdraw");
        println!("4 - Transfer");
        println!("q - back");

        if std::io::stdin().read_line(&mut line).is_err() {
            abort_exit("System input error!");
        }
        line = line.trim_end().to_string();

        if line == String::from(EXIT_CHOICE) {
            return;
        } else if line == String::from("1") {
            pio_bank.saldo();
        } else if line == String::from("2") {
            line.clear();
            println!("How many to deposit?");
            if std::io::stdin().read_line(&mut line).is_err() {
                abort_exit("System input error!");
            }
            line = line.trim_end().to_string().replace(",", ".");
            pio_bank.deposit(Decimal::from_str(&line).unwrap());
            
        } else if line == String::from("3") {
            line.clear();
            println!("How many to withdraw?");
            if std::io::stdin().read_line(&mut line).is_err() {
                abort_exit("System input error!");
            }
            line = line.trim_end().to_string().replace(",", ".");
            pio_bank.withdraw(Decimal::from_str(&line).unwrap());
        } else if line == String::from("4") {
            line.clear();
            println!("Input account to transfer");
            if std::io::stdin().read_line(&mut line).is_err() {
                abort_exit("System input error!");
            }
            let target_account: String = line.trim_end().to_string();
            println!("How many to transfer?");
            line.clear();
            if std::io::stdin().read_line(&mut line).is_err() {
                abort_exit("System input error!");
            }
            line = line.trim_end().to_string().replace(",", ".");
            let result: bool = pio_bank.transfer_to(&target_account, Decimal::from_str(&line).unwrap());
            println!("{}", if result { "Transfer success!" } else { "Transfer failed!" });
        }
    }
}

fn main() {
    println!("{}", BANK_NAME);
    let mut line: String = String::new();
    let mut pio_bank: bank::PioBank = bank::Bank::new();

    while line != String::from(EXIT_CHOICE) {
        line.clear();
        println!("1 - Create account");
        println!("2 - Choose account");
        println!("3 - Show accounts");
        println!("q - quit");
        
        match std::io::stdin().read_line(&mut line) {
            Ok(_n) => {
                line = line.trim_end().to_string();
                if line == String::from("1") {
                    line.clear();
                    println!("Please type your name to create an account[q - quit]:");
                    match std::io::stdin().read_line(&mut line) {
                        Ok(_n) => {
                            line = line.trim_end().to_string();
                            if line == String::from(EXIT_CHOICE) {
                                continue;
                            }

                            if !pio_bank.create_account(&line) {
                                eprintln!("Can not an create account or already exists");
                            }
                        },
                        Err(_error) => {
                            abort_exit("System input error!");
                        }
                    }
                } else if line == String::from("2") {
                    println!("Please type your name to choose an account[q - quit]:");
                    line.clear();
                    match std::io::stdin().read_line(&mut line) {
                        Ok(_n) => {
                            line = line.trim_end().to_string();
                            if line == String::from(EXIT_CHOICE) {
                                continue;
                            }

                            if !pio_bank.set_active_account(&line) {
                                eprintln!("Can not set this account!");
                                continue;
                            }

                            println!("Choosed account {}", line);
                            internal_loop(&mut pio_bank);
                        },
                        Err(_error) => {
                            abort_exit("System input error!");
                        }
                    }
                } else if line == String::from("3") {
                    pio_bank.print_account_names();
                }  else if line == String::from(EXIT_CHOICE) {
                    continue;
                } 
                else {
                    println!("Please try again...");
                    continue;
                }
            },
            Err(_error) => {
                abort_exit("System input error!");
            }
        }
    }

    exit(EXIT_CODE_SUCCESS);
}
