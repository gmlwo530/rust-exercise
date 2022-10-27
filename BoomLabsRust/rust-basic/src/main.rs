enum IsTrue {
    True(u64),
    False,
}

fn main() {
    let my_value = IsTrue::True(100);

    match my_value {
        IsTrue::True(x) => println!("True {}", x),
        IsTrue::False => println!("False"),
    }

    if let IsTrue::True(x) = my_value {
        println!("True {}", x);
    }
}
