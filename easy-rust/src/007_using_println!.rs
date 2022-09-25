// https://www.youtube.com/watch?v=jpLwve-7Cjg
fn give_age() -> i32 {
    42
}

fn main() {
    /*
       macro is function that writes code.
       Using spcific code simply which is made of complicate code
    */
    let my_name = "CHOI";
    let _my_age = 42;
    println!("My name is {} and my age is {}", my_name, give_age());
    println!("My name is {my_name} and my age is {}", give_age());
}
