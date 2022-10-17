// https://www.youtube.com/watch?v=RuTElptHqdg
fn main() {
    println!("This This"); // new line
    print!("This"); // no new line
    print!("This\n");

    println!("c:\\thedriver\\dir");
    println!(r#"c:\thedriver\dir"#); // r#"..."# means raw text

    println!(
        "Let me tell you
어떤 이야기를
봅시다"
    );

    let my_var = &9;
    println!("{:?}", my_var); // :? allows you to format the output in a programmer-facing, debugging context.
    println!("{:p}", my_var); // :p means pointer

    let my_var2 = 9000;
    println!("{:X}", my_var2); // :X means hex
    println!("{:b}", my_var2); // :b means byte

    // Reference: https://doc.rust-lang.org/std/fmt/
    let title = "TODAY'S NEWS";
    println!("{:-^30}", title); // no varialbe name, pad with -, put in cetre, 30 characters long;

    let bar = "|";
    println!("{: <15}{: >15}", bar, bar); // no variable name, pad with space, 15 craracters each, one to the right
    let city1 = "SEOUL";
    let city2 = "TOKYO";
    println!("{a:-<15}{b:->15}", a = city1, b = city2); // variable name city1 and city2, pad with -, one to the left, one the right
}
