#[macro_use]
extern crate scan_fmt;

use std::io::{Result, stdout, Write, Error, ErrorKind};
use std::format;

fn main() -> Result<()> {
    loop {
        print!("Enter # of hours worked (-1 to end): ");
        stdout().flush()?;

        let hours: i32 = match scanln_fmt!("{}", i32) {
            Ok(-1) => break,
            Ok(h) if h <= 0 => {
                let msg = format!("Hours {} is a nonpositive number", h);
                return Err(Error::new(ErrorKind::InvalidInput, msg));
            },
            Ok(h) => h,
            Err(scan_err) => {
                return Err(Error::new(ErrorKind::InvalidInput, scan_err.0));
            },
        };

        print!("Enter hourly rate of the worker ($00.00): ");
        stdout().flush()?;
        let rate: f32 = match scanln_fmt!("{}", f32) {
            Ok(r) if r <= 0.0 => {
                let msg = format!("Hours {} is a nonpositive number", r);
                return Err(Error::new(ErrorKind::InvalidInput, msg));
            },
            Ok(r) => r,
            Err(scan_err) => {
                return Err(Error::new(ErrorKind::InvalidInput, scan_err.0));
            },
        };

        let salary: f32 = if hours > 40 {
            ((hours as f32 - 40.0) * 1.5  + 40.0) * rate
        } else {
            (hours as f32) * rate
        };


        println!("Salary is {}\n", salary);
    };

    Ok(())
}
