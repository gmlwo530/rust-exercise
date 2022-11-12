fn greet(x: u64) {
    println!("[greet] Hello to number {}", x);
}

fn greet2(x: u64) -> u64 {
    println!("[greet2] Hello to number {}", x);
    x
}

fn main() {
    greet(100);

    let a = greet2(100);
    println!("[greet2] Hello to number {}", a);
}
