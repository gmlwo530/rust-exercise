// https://www.youtube.com/watch?v=qHEFgX-zCSs
fn main() {
    /*
       f32 / f64

       Almost, use f64.
    */
    let my_number = 9.; // f64
    let other_number = 9; // i32
    println!("{}", my_number as i32 + other_number); // 18

    let my_number2 = 9.6; // f64
    println!("{}", my_number2 as i32 + other_number); // 18
}
