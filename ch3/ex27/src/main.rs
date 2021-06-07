#[macro_use]
extern crate scan_fmt;

use std::io;
use std::io::{stdout, Write};

fn main() -> io::Result<()> {
    let mut largest1: i32 = i32::MIN;
    let mut largest2: i32 = i32::MIN;

    for number in 1..11 {
        print!("{}. largest1={:>11}, largest2={:>11}, input=", number, largest1, largest2);
        stdout().flush()?;

        let input: i32 = match scanln_fmt!("{}", i32) {
            Ok(i) => i,
            Err(scan_err) => return Err(io::Error::new(io::ErrorKind::InvalidInput, scan_err.0)),
        };

        if input > largest2 {
            largest1 = largest2;
            largest2 = input;
        };
    };

    println!("largest1={}, largest2={}", largest1, largest2);

    Ok(())
}
