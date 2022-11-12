fn greet(x: String) {
    println!("Hello to {}", x);
}

fn main() {
    let greeting: &str = "Hello, wrold!"; // &str은 정적인 값으로 런타임에서 값이 바뀌지 않으며 binary executable에 하드 코딩 됨
    let my_greeting: String = "Hello, wrold!".to_string(); // String 타입은 힙 메모리에 저장 됨. verctor type과 유사함
    greet(my_greeting);
}
