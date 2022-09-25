// https://www.youtube.com/watch?v=f-UCvEs7J3I

fn main() {
    let city = "Seoul";
    let year = 2002;
    let population = 9_987_987;

    println!(
        "The city of {city} in {year} had a population of {population}",
        city = city,
        year = year,
        population = population
    );

    println!(
        "The city of {0} in {1} had a population of {2}. I love {0}!",
        city, year, population
    );

    // string interpolation
    println!("The city of {city} in {year} had a population of {population}");
}
