// https://www.youtube.com/watch?v=MebpscwF7L4
fn main() {
    // Reallocation => Suppose `String` variable has 16 bytes size. When it has size over 16 bytes, variable has more size than original size of string.
    let mut my_name = "".to_string();
    println!("My name is {}", my_name);
    println!(
        "Length is {} and Capacity is: {}",
        my_name.len(),
        my_name.capacity()
    );

    my_name.push_str("David");
    println!("My name is {}", my_name);
    println!(
        "Length is {} and Capacity is: {}",
        my_name.len(),
        my_name.capacity()
    );

    my_name.push('!');
    println!("My name is {}", my_name);
    println!(
        "Length is {} and Capacity is: {}",
        my_name.len(),
        my_name.capacity()
    );

    my_name.push_str(" and I live in Seoul");
    println!("My name is {}", my_name);
    println!(
        "Length is {} and Capacity is: {}",
        my_name.len(),
        my_name.capacity()
    );

    //////////////////////////////////////////////////

    let mut my_name2 = String::with_capacity(26);

    println!("My name is {}", my_name2);
    println!(
        "Length is {} and Capacity is: {}",
        my_name2.len(),
        my_name2.capacity()
    );

    my_name2.push_str("CHOII");
    println!("My name is {}", my_name2);
    println!(
        "Length is {} and Capacity is: {}",
        my_name2.len(),
        my_name2.capacity()
    );

    my_name2.push('!');
    println!("My name is {}", my_name2);
    println!(
        "Length is {} and Capacity is: {}",
        my_name2.len(),
        my_name2.capacity()
    );

    my_name2.push_str(" and I live in Busan");
    println!("My name is {}", my_name2);
    println!(
        "Length is {} and Capacity is: {}",
        my_name2.len(),
        my_name2.capacity()
    );

    // Now size of my_name2 is over 26. So, my_name2 has double size of original size. Thus. 52
    my_name2.push('!');
    println!("My name is {}", my_name2);
    println!(
        "Length is {} and Capacity is: {}",
        my_name2.len(),
        my_name2.capacity()
    );
}
