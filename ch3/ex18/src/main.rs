#[macro_use]
extern crate scan_fmt;

use std::io;
use std::io::{stdout, Write};

fn main() -> io::Result<()>{
    loop {
        print_prompt("Enter account number (-1 to end): ")?;
        let account: i32 = match scanln_fmt!("{}", i32) {
            Ok(-1) => break,
            Ok(a) => a,
            Err(scan_error) => return Err(to_error(scan_error)),
        };

        print_prompt("Enter beginning balance: ")?;
        let begining_balance: f32 = match scanln_fmt!("{}", f32) {
            Ok(b) => b,
            Err(scan_error) => return Err(to_error(scan_error)),
        };

        print_prompt("Enter total charges: ")?;
        let charges: f32 = match scanln_fmt!("{}", f32) {
            Ok(c) => c,
            Err(scan_error) => return Err(to_error(scan_error)),
        };

        print_prompt("Enter credit limit: ")?;
        let credits: f32 = match scanln_fmt!("{}", f32) {
            Ok(c) => c,
            Err(scan_error) => return Err(to_error(scan_error)),
        };

        print_prompt("Enter credit limit: ")?;
        let credit_limit: f32 = match scanln_fmt!("{}", f32) {
            Ok(c) => c,
            Err(scan_error) => return Err(to_error(scan_error)),
        };

        let new_balance: f32 = begining_balance + charges - credits;
        if new_balance <= credit_limit {
            println!();
            continue;
        };

        println!("Account:\t{}", account);
        println!("Credit limit:\t{}", credit_limit);
        println!("Balance:\t{}", new_balance);
        println!("Credit Limit Exceeded.\n");
    };

    Ok(())
}

fn print_prompt(msg: &str) -> io::Result<()> {
    print!("{}", msg);
    stdout().flush()
}

fn to_error(scan_error: scan_fmt::parse::ScanError) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, scan_error.0)
}
