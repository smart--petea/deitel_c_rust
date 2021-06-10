#[macro_use]
extern crate scan_fmt;

use std::io;
use std::io::{stdout, Write};
use regex::Regex;

fn main() -> io::Result<()> {
    print!("binary=");
    stdout().flush()?;

    let mut binary: String = match scanln_fmt!("{}", String) {
        Ok(s) => s,
        Err(scan_error) => return Err(io::Error::new(io::ErrorKind::InvalidInput, scan_error.0)),
    };

    let re = Regex::new(r"^[01]+$").unwrap();
    if re.is_match(&binary) == false {
         return Err(io::Error::new(io::ErrorKind::InvalidInput, "binary should contain only 0s and 1s"));
    }

    let mut decimal: usize = 0;
    let mut ord: usize = 1;

    while binary.chars().count() > 0 {
        if binary.chars().last().unwrap() == '1' {
            decimal = decimal + ord;
        }

        ord = ord * 2;
        binary.pop();
    }

    println!("decimal = {}", decimal);

    Ok(())
}
