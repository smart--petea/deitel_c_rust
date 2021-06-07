fn main() {
    for n in 1..11 {
        print!("{number:>width$}", number=n, width=3);
    }

    println!();
}
