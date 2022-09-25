// https://www.youtube.com/watch?v=jxuNkx4XCXg

// () - empty tuple, unit type (In other language this is mean `void`)

fn number() -> i32 {
    8 // this is mean return. Last statement of function without semicolon
}

// fn empty_tuple() -> () {}
fn empty_tuple() {}

fn main() {
    let number = number();
    println!("Number: {number}");

    let tuple = empty_tuple();
    // println!("{}", tuple); // `()` doesn't implement `std::fmt::Display`
    println!("{:?}", tuple); // debug print

    tuple // return empty tuple
}
