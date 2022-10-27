#[test] // procedural macro 중 attribute macro. 컴파일러로 하여금 밑에 있는 함수는 테스트 코드로 취급하라는 의미
fn my_test() {
    assert!(true);
}

fn main() {
    println!("Hello, world!");
}
