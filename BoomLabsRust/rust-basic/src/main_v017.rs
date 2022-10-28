use std::error::Error;

/*
    pub enum Result<T, E> {
        Ok(T),
        Err(E),
    }
*/

fn process() -> Result<String, Box<dyn Error>> {
    Ok("Hello, world!".to_string())
}

fn main() -> Result<(), Box<dyn Error>> {
    let result = process();

    match result {
        Ok(s) => println!("{}", s),
        Err(e) => println!("Error: {}", e),
    }

    println!("{}", process()?);

    Ok(())
}
