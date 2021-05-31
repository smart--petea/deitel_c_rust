#[macro_use]
extern crate scan_fmt;

use std::io::{Write, stdout};

fn main() {
    let mut counter: i32;
    let mut total: i32;

    total = 0;
    counter = 1;

    while counter <= 10 {
        print!("Enter grade: ");
        if let Err(e) = stdout().flush() {
            panic!("{}", e)
        }

        if let Ok(grade) =  scanln_fmt!("{}", i32) {
            total = total + grade;
        } else {
            panic!("Wrong value introduced");
        }

        counter = counter + 1;
    }

    let average = total / 10;
    println!("Class average is {}", average)
}
