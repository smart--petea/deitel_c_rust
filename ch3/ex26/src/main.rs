fn main() {
    println!("{:<5}{:<5}{:<5}{:<5}", "A", "A+2", "A+4", "A+6");
    for a in (3..15).step_by(3) {
        println!("{:<5}{:<5}{:<5}{:<5}", a, a+2, a+4, a+6);
    }
}
