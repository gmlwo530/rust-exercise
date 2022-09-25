// https://www.youtube.com/watch?v=XfWW4XzT17M
fn print_number(one: i32, two: i32) {
    let multiplied = one * two;
    println!("{}", multiplied);

    let multiplied_by_ten = {
        let first_number = 10;
        first_number * one * two
    };
    println!("{}", multiplied_by_ten);
}

fn main() {
    print_number(9, 8);
}
