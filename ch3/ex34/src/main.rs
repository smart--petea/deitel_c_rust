#[macro_use]
extern crate scan_fmt;

use std::io;
use std::io::{stdout, Write};

fn main() -> io::Result<()> {
    print!("size = "); 
    stdout().flush()?;

    let size: usize = match scanln_fmt!("{}", usize) {
        Ok(0) => return Ok(()),
        Ok(1) => {
            println!("*");
            return Ok(());
        },
        Ok(s) => s,
        Err(scan_error) => return Err(io::Error::new(io::ErrorKind::InvalidInput, scan_error.0)),
    };

    println!("{}", "*".repeat(size));
    for _ in 0..(size - 2) {
        println!("*{}*", " ".repeat(size - 2));
    }
    println!("{}", "*".repeat(size));

    Ok(())
}
