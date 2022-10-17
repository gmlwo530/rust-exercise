// https://www.youtube.com/watch?v=es_LX2vdCbo
fn main() {
    /*
        String => growable string / owned type(variable has own data) / sized type / in heap
        &str => ref str "string slice" / dynamic type / in stack

        `String` is more comfortable than `&str`
    */

    let my_name = "David"; // &str
    let my_name_2 = "David".to_string(); // String
    let other_my_name = String::from("David"); // String
                                               // growable + shrinkable
    let mut my_other_name = "David3".to_string();
    my_other_name.push('!');
    println!("{}", my_other_name)
}
