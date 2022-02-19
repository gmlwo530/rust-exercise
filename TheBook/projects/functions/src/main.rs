fn main() {
    println!("Hello, world!");

    another_function(5, 'h');

    let x = five();
    println!("This is five: {}", x);
}

// 함수 인자에는 타입을 반드시 명시 해줘야 한다.
fn another_function(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn five() -> i32 {
    5
}