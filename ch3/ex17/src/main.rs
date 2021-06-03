#[macro_use]
extern crate scan_fmt;

use std::io;
use std::io::{stdout, Write};

fn main() -> io::Result<()> {
    loop {
        print!("Enter the gallons used (-1 to end):");
        stdout().flush()?;

        let galons: f32 = match scanln_fmt!("{}", f32) {
            Ok(-1.0) => break,
            Ok(g) => g,
            Err(scan_error) => return Err(io::Error::new(io::ErrorKind::InvalidInput, scan_error.0)),
        };
    };

    Ok(())
}
