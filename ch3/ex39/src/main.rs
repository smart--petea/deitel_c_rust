#[macro_use]
extern crate scan_fmt;

use std::io;
use std::io::{stdout, Write};

fn main() -> io::Result<()> {
    print!("number = ");
    stdout().flush()?;

    let mut number: u32 = scanln_fmt!("{}", u32).unwrap();
    let mut count_7s: usize = 0;

    loop {
        if number % 10 == 7 {
            count_7s = count_7s + 1;
        }

        number = number / 10;

        if number == 0 {
            break;
        }
    }

    println!("count_7s = {}", count_7s);

    Ok(())
}
