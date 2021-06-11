fn main() {
    for i in 0..9 {
        if i % 2 == 1 {
            print!(" ");
        }
        println!("{}", "* ".repeat(8));
    }
}
