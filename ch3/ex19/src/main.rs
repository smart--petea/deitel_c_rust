#[macro_use]
extern crate scan_fmt;

use std::io;
use std::io::{stdout, Write};

fn main() -> io::Result<()> {
    loop {
        print_prompt("Enter sales in dollars (-1 to end): ")?;
        let sells: f32 = match scanln_fmt!("{}", f32) {
            Ok(-1.0) => break,
            Ok(s) => s,
            Err(scan_error) => return Err(io::Error::new(io::ErrorKind::InvalidInput, scan_error.0)),
        };

        println!("Salary is: ${}\n", 200.00 + 0.09 * sells);
    };

    Ok(())
}

fn print_prompt(prompt: &str) -> io::Result<()> {
    print!("{}", prompt);
    stdout().flush()
}
