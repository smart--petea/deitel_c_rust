#[macro_use]
extern crate scan_fmt;

use std::io;
use std::io::{stdout, Write};

fn main() -> io::Result<()> {
    let mut number: u32 = 0;
    let mut largest: i32 = i32::MIN; 
    
    while number < 10 {
        print!("{}. largest={:>11},  input = ", number, largest);
        stdout().flush()?;

        let input: i32 = match scanln_fmt!("{}", i32) {
            Ok(i) => i,
            Err(scan_error) => return Err(io::Error::new(io::ErrorKind::InvalidInput, scan_error.0)),
        };

        if largest < input {
            largest = input;
        };

        number = number + 1;
    };

    println!("\nlargest={}", largest);

    Ok(())
}
