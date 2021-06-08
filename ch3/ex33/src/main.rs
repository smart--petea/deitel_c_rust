#[macro_use]
extern crate scan_fmt;

use std::io;
use std::io::{stdout, Write};

fn main() -> io::Result<()> {
    print!("size=");
    stdout().flush()?;
    let size: usize = match scanln_fmt!("{}", usize) {
        Ok(0) => return Ok(()),
        Ok(s) if s > 20 => return Err(io::Error::new(io::ErrorKind::InvalidInput, "size should take values from range [0,20]")),
        Ok(s) => s,
        Err(scan_err) => return Err(io::Error::new(io::ErrorKind::InvalidInput, scan_err.0)),
    };

    let row = "*".repeat(size);
    for _ in 0..size {
        println!("{}", row);
    };

    Ok(())
}
