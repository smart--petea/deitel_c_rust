#[macro_use]
extern crate scan_fmt;

use std::io;
use std::io::{stdout, Write};

fn main() -> io::Result<()> {
    print!("number = ");
    stdout().flush()?;

    match scanln_fmt!("{}", usize) {
        Ok(n) if n <= 9999 || n >= 100000 => return Err(io::Error::new(io::ErrorKind::InvalidInput, "number should be of 5 digit length")),
        Ok(n) => {
            if palindrome(n) {
                println!("palindrome - YES");
            } else {
                println!("palindrome - NO");
            }

            Ok(())
        },
        Err(scan_err) => return Err(io::Error::new(io::ErrorKind::InvalidInput, scan_err.0)),
    }
}

fn palindrome(number: usize) -> bool {
    let number_digits: usize = (number as f64).log10().floor() as usize + 1;
    let mut divisor: f64 = 10.0.powi(number_digits - 1);

    while divisor > 1.0 {
        let upper_digit: usize = (number / divisor).floor() as usize;
        let lower_digit: usize = (number % divisor);
        if upper_digit != lower_digit {
            return false;
        }

        divisor = divisor / 10;
    }

    true
}
