// https://www.youtube.com/watch?v=uHuMJw73ukg
fn main() {
    let my_number = 10;
    /*
       rust is immutable by default

       my_number = 9; // error
    */

    let mut mut_my_number = 10;
    mut_my_number = 9;

    let my_variable = 10;
    let my_variable = "My variable"; // shadowing

    // why is exist shadowing?
    let x = 9;
    let x = x * 2;
    let x = x * 3;
    println!("x: {}", x);

    let my_variable = 9;
    {
        let my_variable = "Some string";
        println!("{}", my_variable); // Some string
    }
    println!("{}", my_variable); // 9
}
