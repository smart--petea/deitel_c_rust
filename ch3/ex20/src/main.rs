#[macro_use]
extern crate scan_fmt;

use std::io::{stdout, Write, Error, ErrorKind, Result};
use std::format;

fn main() -> Result<()> {
    loop {
        print!("Enter loan principal (-1 to end): ");
        stdout().flush()?;

        let loan: f32 = match scanln_fmt!("{}", f32) {
            Ok(-1.0) => break,
            Ok(l) if l <= 0.0 => {
                let msg: String = format!("Loan {} is negative or zero", l);
                let err = Error::new(ErrorKind::InvalidInput, msg);
                return Err(err);
            },
            Ok(l) => l,
            Err(scan_error) => {
                let err = Error::new(ErrorKind::InvalidInput, scan_error.0);
                return Err(err);
            },
        };

        print!("Enter interest rate: ");
        stdout().flush()?;

        let rate: f32 = match scanln_fmt!("{}", f32) { Ok(r) if r <= 0.0 => {
                let msg: String = format!("Rate {} is negative or zero", r);
                let err = Error::new(ErrorKind::InvalidInput, msg);
                return Err(err);
            },
            Ok(r) => r,
            Err(scan_error) => {
                let err = Error::new(ErrorKind::InvalidInput, scan_error.0);
                return Err(err);
            },
        };

        print!("Enter term of the loan in days: ");
        stdout().flush()?;
        let term: u32 = match scanln_fmt!("{}", u32) {
            Ok(t) => t,
            Err(scan_error) => {
                let err = Error::new(ErrorKind::InvalidInput, scan_error.0);
                return Err(err);
            },
        };

        let charge: f32 = loan * rate * (term as f32) / 365.0;
        println!("The interest charge is ${}\n", charge);
    }

    Ok(())
}
