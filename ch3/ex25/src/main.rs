fn main() {
    println!("{:<8}{:<8}{:<8}{:<8}", "N", "10*N", "100*N", "1000*N");
    for n in 1..11 {
        println!("{:<8}{:<8}{:<8}{:<8}", n, n * 10, n * 100, n * 1000);
    }
}