// https://www.youtube.com/watch?v=E48iJShSi7s
const HIGH_SCORE: i32 = 20; // 'const' declaration should have type annotation / global scope
static LOW_SCORE: i32 = 0; // 'static' use same memory / global scope

/*
    static mut LOW_SCORE: i32 = 0;

    This is unsafe statement
    unsafe statement is not used.
*/

fn print_high_score() {
    println!("The high score is {}", HIGH_SCORE);
}

fn main() {
    let x = 8; // 'let' binding

    print_high_score();
}
