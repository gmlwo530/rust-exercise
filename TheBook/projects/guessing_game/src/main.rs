/*
    std는 standard library다.
    기본적으로 rust에는 standard library에 정의 된 몇몇 아이템들은 모든 프로그램 영역에 가져와진다. 이걸 prelude라고 부른다.
    만약 prelude에 정의되어 있지 않은 것을 사용하기 위해서는 use 키워드를 사용해서 명시해야 한다.
*/
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new(); // String 타입의 empty instance를 만든다. `::` 문법은 new가 String 타입과 관련된 함수임을 의미한다.

    io::stdin()
        .read_line(&mut guess) // & 문법은 참조를 의미한다.
        .expect("Failed to read line");
        // read_line 메서드는 io::Result의 인스턴스를 반환한다. 이 인스턴스는 ok와 err를 값으로 가지고 있는 enum 타입이고 expect 라는 메서드를 가지고 있다.
        // 만약 값이 err고 expect 함가 실행 되면 프로그램이 종료되고 해당 메시지를 출력한다. 만약 expect 메서드를 실행하지 않으면 컴파일러는 컴파일 시 에러를 일으킨다.

    
    
    // 무한 루프
    loop {
        /*
            `: u32` 로 타입 명시를 해줘서 parse 함수가 변경 할 타입을 명시해준다. 만약 annotating 없이 사용하고 싶다면 두 번째 줄과 같이 하면 된다.
            let guess: u32 = guess.trim().parse().expect("Please type a number!");
            let guess = guess.trim().parse::<u32>()

            parse가 리턴하는 Result 인스턴스를 match 문법으로 예외 처리해주기 위해서 아래와 같이 작성한다.
        */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
