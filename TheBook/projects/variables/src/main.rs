fn main() {
    // let x = 5; -> 5번째 라인에서 컴파일 에러 발생함.
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
