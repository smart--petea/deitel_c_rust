#[macro_use]
extern crate scan_fmt;

use std::io;
use std::io::{stdout, Write};

fn main() -> io::Result<()> {
    const PI: f32 = 3.14159;

    print!("radius = ");
    stdout().flush()?;
    let radius: f32 = scanln_fmt!("{}", f32).unwrap();
    if radius < 0.0 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Radius should be a number nonnegative"));
    }

    println!("diameter = {}", 2.0 * radius);
    println!("circumference = {}", 2.0 * radius * PI);

    Ok(())
}
