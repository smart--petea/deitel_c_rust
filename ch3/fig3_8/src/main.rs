#[macro_use]
extern crate scan_fmt;

use std::io::{Write, stdout};

fn main() {
    let average: f32;

    let mut total: i32 = 0;
    let mut counter: i32 = 0;

    print!("Enter grade, -1 to end: ");
    if let Err(e) = stdout().flush() {
        panic!("{}", e);
    }

    loop {
        match scanln_fmt!("{}", i32) {
            Ok(-1) => break,
            Ok(grade) => {
                total = total + grade;
                counter = counter + 1;
            },
            Err(e) => panic!("{}", e),
        };

        print!("Enter grade, -1 to end: ");
        if let Err(e) = stdout().flush() {
            panic!("{}", e);
        }
    }; 

    if counter != 0 {
        average = (total as f32) / counter as f32;
        println!("Class average is {:.2}", average);
    } else {
        println!("No grades were entered")
    }
}
