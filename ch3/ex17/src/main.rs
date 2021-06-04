#[macro_use]
extern crate scan_fmt;

use std::io;
use std::io::{stdout, Write};

fn main() -> io::Result<()> {
    let mut miles_total: i32 = 0; 
    let mut gallons_total: f32 = 0.0; 

    loop {
        print_prompt("Enter the gallons used (-1 to end): ")?;

        let gallons: f32 = match scanln_fmt!("{}", f32) {
            Ok(-1.0) => break,
            Ok(g) => g,
            Err(scan_error) => return Err(io::Error::new(io::ErrorKind::InvalidInput, scan_error.0)),
        };

        print_prompt("Enter the miles driven: ")?;
        let miles: i32 = match scanln_fmt!("{}", i32) {
            Ok(m) => m,
            Err(scan_error) => return Err(io::Error::new(io::ErrorKind::InvalidInput, scan_error.0)),
        };

        println!("The miles / gallon for this tank was {}\n", miles as f32 / gallons);

        miles_total += miles;
        gallons_total += gallons;
    };

    println!("The overall average miles/gallon was {}\n", miles_total as f32 / gallons_total);
    Ok(())
}

fn print_prompt(prompt: &str) -> io::Result<()> {
    print!("{}", prompt);
    stdout().flush()
}
