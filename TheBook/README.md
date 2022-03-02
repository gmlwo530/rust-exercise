[Note]: 이 README는 문서에서 필요한 부분만 메모한 내용임. 더 자세한 내용을 보고 싶으면 문서를 참고 바람.
 
# [Cargo](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)

- Rust에서는 코드의 패키지들을 crate라고 부른다
- cargo를 사용하면 소스파일은 모두 src 디렉토리 안에 있어야 한다.

## 명령어

### 프로젝트 생성

```bash
# cargo로 프로젝트 생성하기(new 명령어는 .gitignore 파일과 함께 git repository를 시작한다)
$ cargo new hello_cargo

# 아래 명령어로 이미 git repository인 프로젝트를 오버라이딩 할 수 있다.
$ cargo new hello_cargo --vsc=git
```

### 프로젝트 빌드

```bash
# ./target/debug 디렉토리에 실행 파일이 생성 된다.
$ cargo build
```

### 프로젝트 빌드 & 실행

```bash
# 프로젝트를 빌드하고 바로 실행한다
$ cargo run
```

### 프로젝트 실행 가능 체크

```bash
# 실행 파일을 생성하지 않고 컴파일 한다.
# 실행 파일을 생성하지 않음으로써 build 명령어보다 빠르게
# 프로젝트가 컴파일 될 수 있는지 확인 할 수 있다.
$ cargo check
```

### 프로젝트 릴리즈

```bash
# 릴리즈 용 실행 파일을 target/release 디렉토리에 생성한다.
# 벤치마킹을 할거면 이 실행 파일로 해야한다.
$ cargo build --release
```

# [Programming a Guessing Game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)

간단한 프로젝트를 만들면서 rust의 근본적인 것들을 알아본다.

## Cargo.toml 파일 업데이트하기

```jsx
rand = "0.8.3"
```

위 코드는 `^0.8.3` 의 축약 표현이다. 최소 0.8.3 버전을 사용하고 업데이트 버전이 있으면 0.9.0 이하의 버전까지는 사용하겠다는 뜻이다.

자세한 코드 설명은 [프로젝트 참조](https://github.com/gmlwo530/rust-exercise/tree/main/TheBook/projects/guessing_game)

# [Common Programming Concepts](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html#common-programming-concepts)

## Variables and Mutability

- Rust의 기본 변수는 불변이다.
- 때로는 mut 키워드를 붙혀서 퍼포먼스를 가져갈 수 있다.
- 예를 들어 크기가 큰 데이터 구조라면 가변적인 인스턴스를 사용하는 것이 값을 복사하고 새로운 인스턴스에 할당하는 것보다 빠르다.
반대로 작은 데이터 구조라면 새로운 인스턴스를 만들고 쓰는게 더 함수지향의 프로그래밍을 하고 생각하기 쉬운 구조를 만들 수 있다. 이러한 명확성을 얻기 위해 낮은 성능은 가치 있는 패털티가 될 수 있다.

## Constants

- Constant는 immutable variable 처럼 값을 변경 할 수 없다. 대신 몇 가지 차이점이 있다.
    1. mut 키워드를 붙힐 수 없다. const는 무조건 불변이다. let을 mut와 조합하여 가변 변수를 만들 있는 것과 반대로 무조건 불면이다.
    2. 데이터 타입을 항상 무조건 명시해야 한다.
    3. 상수는 글로벌 스코프를 포함해서 모든 스코프에 선언 될 수 있다.
    4. 상수에 런타임에 계산 된 값이 할당 될 수 없다. 오직 상수 표현의 값만 할당 될 수 있다.
        
        ```rust
        // 상수 표현 예시
        // rust의 상수 변수 이름의 컨벤션은 아래와 같다.
        const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // 이 수식은 컴파일 타임에 계산 됨.
        ```
        
- 상수는 선언된 범위 내에서 프로그램이 실행되는 전체 시간 동안 유효하다.

## Shadowing

아래와 같이 불변 변수를 let으로 다시 정의하는 것을 shadowing이라고 한다.

```rust
fn main() {
    let x = 5;

    let x = x + 1; // 6

    {
        let x = x * 2; // 12
        println!("The value of x in the inner scope is: {}", x); // The value of x in the inner scope is: 12
    }

    println!("The value of x is: {}", x); // The value of x is: 6
}
```

shadowing이 mut 변수와 다른 점은 새로운 변수를 만든 다는 것이다. 이로 인해 같은 이름을 타입이 다른 변수를 만들 수 있는 장점이 있다.

## [Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)

- Rust에는 scalar와 compound라는 두 가지 데이터 타입이 있다.
- Rust는 정적 타입 언어이다. 즉 컴파일 타임에 모든 변수는 타입이 정해져 있어야 한다는 의미다.
- 개발자가``parse` 같은 메소드로 타입 변환을 한다면 반드시 타입 명시(annotation)를 해줘야 한다. 만약 하지 않으면 컴파일 타임 때 에러가 난다.

### Scalar Types

- 하나의 값을 표현하는 타입
- 정수, 소수, Boolean, character

**Integer Types**

- `i` : 부호가 있는 정수, `u` : 부호가 없는 정수
- 1_000 과 1000은 같은 숫자. 앞은 읽기 쉬운 표기법
- rust의 기본 정수 타입은 i32
- usize, isize 타입은 컴퓨터 아키텍처에 따라서 크키가 정해짐. 64 비트 아키텍처의 컴퓨터이면 64 비트 타입이 됨.
- **정수 오버플로우**
    - 디버그 모드에서 컴파일 시 정수 오버플로우가 생기면 panic이 발생함
    - 릴리스 모드에서 컴파일 시(`—release` 플래그 사용)  정수 오버플로우를 체크하지 않음. 오버플로우가 발생하면 panic 시키지 않음. 대신 타입의 최대값보다 큰 값만큼 “wrap around”라는 처리를 함. 예를 들어 u8 타입에서 256은 0으로, 257은 1로 변환함. *(번역 부족)*
    - 이를 핸들링하기 위해서 스탠다드 라이브러리에서 제공해주는 오버 플로우 관련 메소드들을 사용하면 됨. ([예시](https://doc.rust-lang.org/std/?search=overflowing))

**Floating-Point Types**

- f32, f64 두 가지 타입이 있다.
- f64 타입은 현재 CPU로 같은 속도와 높은 정확성을 가지고 있기 때문에 기본 소수 타입으로 사용 된다.

**The Boolean Type**

```rust
fn main() {
	let t = true;
	let f: bool = false;
}
```

**The Character Type**

```rust
fn main() {
	let c = 'z'
}
```

- single quote로 선언 해야 한다. double quote를 사용하면 string 타입이 된다.
- rust의 char 타입은 Unicode Scalar Value를 표현 할 수 있는 4바이트의 크기를 가진다.

### Compound Types

- 다수의 값을 가짐
- tuple과 array 타입이 있다.

**The Tuple Type**

- 다양한 타입의 값을 그룹화 하는 타입
    
    ```rust
    fn main() {
        let tup: (i32, f64, u8) = (500, 6.4, 1);
    }
    ```
    
- 고정 된 길이를 갖고 한 번 선언하면 사이즈를 키우거나 줄일 수 없다.
- 아래와 같이 비구조화를 할 수 있다
    
    ```rust
    fn main() {
        let tup = (500, 6.4, 1);
    
        let (x, y, z) = tup;
    		/*
    			let (.., z) = tup;
    			let (_, y, _) = tup;
    			let (x, ..) = tup;
    			와 사용하지 않을 변수는 할당하지 않을 수 있다.
    		*/
    
        println!("The value of y is: {}", y);
    
    		let one = tup.0 // index로 값을 조회한다.
    }
    ```
    

**The Array Type**

- 모든 요소는 같은 타입이다.
- Rust의 array는 고정 길이다.
- 아래와 같이 타입 어노테이션과 함께 요소의 개수를 적어줄 수 있다.
    
    ```rust
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    ```
    
- 축약해서 선언 가능하다.
    
    ```rust
    let a = [3; 5]; // let a = [3, 3, 3, 3, 3]의 축약
    ```

## Functions

- rust에서는 함수가 정의 된 위치는 상관 없다.
- 함수 인자에는 타입을 반드시 명시 해줘야 한다.

### Statement and Expression

- 함수는 여러 줄의 statement와 조건적으로 맨 마지막의 expression 줄로 이루어져 있다.
- 러스트는 expression-based language이므로, 다른 언어와 달리 statement와 expression의 차이점이 중요하다.
- **Statement**는 어떤 행동을 지시하고 값을 반환하지 않는다. **Expressions**는 결과 값을 평가한다. 예제로 자세히 알아보자
    
    ```rust
    // 함수 정의는 statement다.
    fn main() {
    		let y = 6; // 이 라인은 statement다.
    }
    ```
    
    ```rust
    // statement는 값을 반환하지 않으므로 아래 코드는 동작하지 않는다.
    fn main() {
        let x = (let y = 6);
    }
    ```
    
    Expressions는 값을 평가하는 것이다. 예를 들어 `5+6` 같은 수학 계산을 생각 해 볼 수 있다. 함수 호출, 매크로 호출, curly brakets로 만들어진 새로운 범위 블록은 expression이다.
    
    ```rust
    fn main() {
        let y = {
            let x = 3;
            x + 1
        };
    
        println!("The value of y is: {}", y);
    }
    ```
    
    ```rust
    // 위의 코드에서 아래는 expression이다.
    {
        let x = 3;
        x + 1 // 이 라인을 보면 세미콜론이 없다. expression은 끝에 세미콜론을 가지지 않는다. 만약 세미콜론을 붙히면 이 라인은 statement가 된다.
    }
    ```
    

### Functions with Return Values

- 리턴하는 타입을 → 옆에 적어준다.
- 함수의 반환 값은, 함수의 마지막 expression이다.
- 만약 값을 좀 더 일찍 반환하고 싶으면(함수의 마지막 expression에 다다르기 전에), `return` 키워드를 사용하면 된다.

```rust
fn five() -> i32 {
    5
}
```

## Comments

- `//` 키워드로 주석을 남길 수 있다.
- 또 다른 종류의 주석(문서화 할 수 있는 주석 등)은 이후에 학습한다.

## Control Flow

- `if` 표현식과 쓰이는 코드 블락을 *arms*라고 부른다.
    
    ```rust
    fn main() {
        let number = 3;
    
        if number {
            println!("number was three");
        }
    }
    ```
    
- 조건에는 bool 아닌 타입이 들어가면 에러가 난다.
- `if` 는 표현식이므로 할당이 가능하다.
    
    ```rust
    fn main() {
        let condition = true;
        let number = if condition { 5 } else { 6 };
    
        println!("The value of number is: {}", number);
    }
    ```
    
    코드 블락은 마지막 expression을 조사한다. 그래서 같이 쓰이는 블록에서는 같은 타입의 결과를 반환하는 표현식이 사용 되야 한다. 만약 아래와 같이 작성하면 오류가 난다.
    
    ```rust
    let number = if condition { 5 } else { "six" }; // rust의 변수는 하나의 타입만 갖는다.
    ```
    

### 반복문

- 반복문을 작성 할 수 있는 키워드는  `loop, while, for`
    
    ```rust
    fn main() {
    		loop {
    			println!("again");
    		}
    }
    ```
    
- loop를 중첩으로 쓰면 break와 continue는 제일 가까운 반복문에 작동 된다.
- loop에 라벨링을 하면 어떤 loop에 break와 continue를 작동 시킬지 정할 수 있다.
    
    ```rust
    fn main() {
        let mut count = 0;
        'counting_up: loop { // 라벨 앞에 '을 붙혀줘야 한다.
            println!("count = {}", count);
            let mut remaining = 10;
    
            loop {
                println!("remaining = {}", remaining);
                if remaining == 9 {
                    break;
                }
    
                if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            }
    
            count += 1;
        }
    
        println!("End count = {}", count);
    }
    ```
    
- loop 변수를 이용해서 값을 반환 할 수 있다.
    
    ```rust
    fn main() {
        let mut counter = 0;
    
        let result = loop {
            counter += 1;
    
            if counter == 10 {
                break counter * 2; // ;을 붙힌 걸 주의하자.
            }
        }; // ;을 붙힌 걸 주의하자.
    
        println!("The result is {}", result);
    }
    ```
    
- while 문을 쓰면 if문과 loop의 조합을 좀 더 간소화 할 수 있다.
    
    ```rust
    fn main() {
        let mut number = 3;
    
        while number != 0 {
            println!("{}!", number);
    
            number -= 1;
        }
    
        println!("LIFTOFF!!!");
    }
    ```
    
- **collection과 반복문**
    
    ```rust
    fn main() {
        let a = [10, 20, 30, 40, 50];
    
        for element in a {
            println!("the value is: {}", element);
        }
    }
    ```
    
- **Range loop 돌기**
    
    ```rust
    fn reverse_loop_range() {
        for number in (1..4).rev() {
            println!("{}!", number);
        }
        println!("LIFTOFF!!!");
    }
    ```

# Understanding Ownership

- 가비지 콜렉터 없이 메모리 안정성을 보장을 가능하게 해주는 특징이다.

## What is Ownership?

- ownership은 Rust 프로그램이 메모리를 관리하는 방법을 제어하는 규칙이다.
- 일련의 규칙이 있는 소유권 시스템이 메모리를 관리한다.
- 컴파일러는 이 규칙들을 확인하고, 만약 위험한 규칙들이 있으면 컴파일 되지 않는다.

### The stack and the heap

- ownership을 설명하기 위해 stack과 heap에 대해 미리 알아야 된다.
- 둘 다 런타임에 우리의 코드가 사용 할 수 있는 메모리의 일부인데, 다른 방식으로 구조화 되어 있다.
- stack은 선입선출이다.
- stack에 저장 되는 데이터들은 사이즈가 고정 되어 있어야 한다. 컴파일 타임에 데이터의 사이즈를 알 수 없는 데이터 또는 사이즈가 변할 수 있는 데이터는 heap에 저장 되어야 한다.
- heap에 데이터를 집어 넣을 때 확실한 데이터 공간을 요청하게 된다.
- memory allocator는 heap에서 충분한 사이즈의 비어있는 스팟을 찾고, 이 스팟을 사용한다고 표시한다. 그리고 pointer를 리턴하는데 이것은 위치의 주소다. 이 과정을 allocating on the heap 또는 allocating이라고 부른다. pointer는 고정 된 사이즈를 가지므로 stack에 저장 할 수 있다.
- stack은 heap 처럼 새로운 데이터를 저장하기 위해 공간을 찾고 공간을 bookkeeping 할 필요가 없어서 빠르다.
- heap은 pointer를 따라 데이터를 찾아야 하기 때문에 stack 보다 데이터에 접근하는 시간이 오래 걸린다. 대신 접근해야 할 데이터가 서로 가까이 있으면 일시적으로 빨라질 수 있다.
- ownership의 역할은 heap에 어떤 데이터가 있고, 중복된 데이터를 최소화 하고, 사용하지 않는 데이터를 지우는데 있다.
- onwership을 이해하면 stack과 heap을 신경 써야 할 필요가 없지만, ownership의 주요 목적이 heap을 관리하는 것이라는 내용을 알고 있으면 ownership의 동작 방식을 이해 할 때 도움이 된다.

### Ownership Rules

- Rust의 각 값은 owner라고 불리는 변수를 가지고 있다.
- 한 번에 하나의 owner만 있다.
- owner가 scope를 벗어 났을 때, 값은 삭제 된다.

### The String Type

- ownership의 규칙을 설명하기 위해서 이전에 배웠던 타입보다 복잡한 String type을 알아볼거다.
- 이전에 배운 타입들은 고정 된 사이즈를 가지고 있어서 stack에 저장 될 수 있었다.
- 하지만 head에 저장 된 데이터를 찾고 Rust가 어떻게 데이터를 지우는 알기 위해서 String 타입은 좋은 예시가 된다.
- String과 ownership의 관계는 다른 복잡한 타입도 가지고 있다.
- String은 아래와 같이 사용한다.
    
    ```groovy
    let mut s = String::from("hello");
    
    s.push_str(", world!"); // push_str() appends a literal to a String
    
    println!("{}", s); // This will print `hello, world!`
    ```
    
    String은 가변적이고 literal은 왜 불가변일까? 이것은 두 타입이 memory를 어떻게 처리하는지에 차이점이 있어서 그렇다.

    ### Memory and Allocation

- `String` 타입을 사용하기 위해서 heap에 특정 메모리 양을 할당한다. 이 뜻은 아래와 같다
    - 런타임에 메모리 할당자에서 메모리를 요청해야 한다. `String::from` 호출 할 때 이 과정이 진행 된다.
    - String 사용이 끝나면 메모리를 할당자에게 반납해야 한다.
- 다른 언어들은 GC를 사용하거나 코드로 명시적으로 메모리를 할당하고 반납한다. 러스트는 이와 다르게 변수가 scope를 벗어 났을 때 메모리 자동으로 반환 된다.
- 변수가 scope 범위에서 벗어나면, Rust `drop` 이라는 특별한 함수를 호출 한다. 메모리를 반환하는 코드를 넣을 수 있는 곳이다. 중괄호가 닫히는 부분에서 Rust는 이 `drop`을 자동으로 호출한다.

### Ways Variables and Data Interact: Move

- 아래와 같이 코딩하면 x와 y는 고정 된 값인 5라는 값을 갖고, 두 값은 stack에 들어간다.
    
    ```rust
    let x = 5;
    let y = x;
    ```
    
- 그런데 위와 같은 형태로 String 타입을 다루면 위와 다르게 동작한다.
    
    ```rust
    let s1 = String::from("hello");
    let s2 = s1;
    ```
    
    일단 String 타입이 어떻게 이루어져 있는지 확인해보자.
    
    ![Untitled](https://doc.rust-lang.org/book/img/trpl04-01.svg)
    
    capacity는 heap 요청하는 메모리 사이즈로 byte 단위다.
    
    포인터, 길이, 크기는 stack에 저장되고, 오른쪽의 string의 내용은 heap에 저장된다.
    
    위 코드처럼 값은 복사하면 포인터가 복사 되어 아래와 같이 s1와 s2가 가지고 있는 포인터는 heap에 저장 된 같은 데이터를 가르키게 된다.
    
    ![Untitled](https://doc.rust-lang.org/book/img/trpl04-02.svg)
    
    Rust는 heap에 있는 데이터도 복사하지 않는다.
    
    앞선 상황에서 rust는 변수가 scope를 벗어나면 drop 함수를 호출하고 heap에 있는 메모리를 삭제 한다고 했다. 위와 같이 두 변수가 같은 데이터를 가르키게 되면 같은 메모리에 대해 두 번의 메모리 반납 과정이 일어나게 된다. 이런 중복 된 과정은 잠재적인 보안 취약성을 유발 할 수 있는 메모리 손상을 일으킨다.
    
    이 상황을 만들지 않기 위해서, rust는 `let s2 = s1` 줄 이후 s1은 더 이상 유효하지 않다고 판단한다. 그렇기 때문에 아래의 코드는 동작하지 않는다.
    
    ```rust
    let s1 = String::from("hello");
    let s2 = s1;
    
    println!("{}, world!", s1);
    ```
    
    이것은 다른 언어의 얕은 복사 개념과 유사해 보인다. 하지만 s1은 더 이상 유효하지 않으므로 이 과정을 move라고 부른다.
    
    이렇게 디자인 선택에서 추측 해볼 수 있는 점은, rust는 자동으로 deep copy를 하지 않는 다는 점이다. 
    

### Ways Variables and Data Interact: Clone

- Deep copy를 하고 싶다면 `clone` 이라는 함수를 쓰면 된다.
    
    ```rust
    let s1 = String::from("hello");
    let s2 = s1.clone();
    
    println!("s1 = {}, s2 = {}", s1, s2);
    ```
    
- 물론 이 연산은 비싼 연산이다

### Ownership and Functions

- 함수에 어떤 인자를 넣느냐에 따라 move 또는 copy 과정이 일어난다.
    
    ```rust
    fn main() {
        let s = String::from("hello");  // s comes into scope
    
        takes_ownership(s);             // s's value moves into the function...
                                        // ... and so is no longer valid here
    
        let x = 5;                      // x comes into scope
    
        makes_copy(x);                  // x would move into the function,
                                        // but i32 is Copy, so it's okay to still
                                        // use x afterward
    
    } // Here, x goes out of scope, then s. But because s's value was moved, nothing
      // special happens.
    
    fn takes_ownership(some_string: String) { // some_string comes into scope
        println!("{}", some_string);
    } // Here, some_string goes out of scope and `drop` is called. The backing
      // memory is freed.
    
    fn makes_copy(some_integer: i32) { // some_integer comes into scope
        println!("{}", some_integer);
    } // Here, some_integer goes out of scope. Nothing special happens.
    ```
    

### [Return Values and Scope](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#return-values-and-scope)

- 함수에서 반환하는 것도 할당하는 것처럼 값이 move한다.
    
    인자로 넘기면 값이 이동하고, 인자로 받은 값이 반환 되면 다시 값이 함수 밖으로 이동 된다.
    
    ```rust
    fn main() {
        let s2 = String::from("hello");     // s2 comes into scope
    
        let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                            // takes_and_gives_back, which also
                                            // moves its return value into s3
    }
    
    // This function takes a String and returns one
    fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                          // scope
    
        a_string  // a_string is returned and moves out to the calling function
    }
    ```
    
- 만약 함수가 받은 값을 그대로 넘기기 위해서는 아래와 같이 번잡한 구조로 프로그래밍 해야 한다.
    
    ```rust
    fn main() {
        let s1 = String::from("hello");
    
        let (s2, len) = calculate_length(s1);
    
        println!("The length of '{}' is {}.", s2, len);
    }
    
    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len(); // len() returns the length of a String
    
        (s, length)
    }
    ```
    
    Rust에는 이런 번잡한 구조를 해소하기 위해서 ownership 전달 없이 값을 넘길 수 있는 references라는 개념이 존재한다.

## [References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

- reference는 포인터와 비슷하다. 대신, 포인터와 다르게 reference는 정확한 타입의 유효한 값을 가르킨다.
- 아래 코드의 변수가 담고 있는 데이터와 가르키는 방향은 사진과 같다
    
    ```rust
    fn main() {
        let s1 = String::from("hello");
    
        let len = calculate_length(&s1);
    
        println!("The length of '{}' is {}.", s1, len);
    }
    
    fn calculate_length(s: &String) -> usize {
        s.len()
    }
    ```
    
    ![Untitled](https://doc.rust-lang.org/book/img/trpl04-05.svg)
    
    - & 문법은 s1에 대한 refer 만드는 문법이다.
    - &String은 s 변수가 String에 대한 reference라는 표현이다.
    - s는 ownership을 가진적이 없으므로 함수가 끝나도 ownership을 넘기다든지 하는 일이 벌어지지 않는다.
- reference를 만드는 행위를 borrowing(빌리다)라고 한다.
- 당연히 reference를 사용해서 값을 수정하는 것은 불가능하다.
    
    ```rust
    fn change(some_string: &String) {
        some_string.push_str(", world"); // error!
    }
    ```
    

### Mutable References

- reference를 사용해서 값을 수정할려면, mutable reference를 이용하면 된다.
    
    ```rust
    fn main() {
        let mut s = String::from("hello");
    
        change(&mut s);
    }
    
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }
    ```
    
- mutable reference를 쓰는 하나의 제약 조건은 정확한 순간에 오직 하나의 mutable reference만 정의 할 수 있다.
    
    ```rust
    let mut s = String::from("hello");
    
    let r1 = &mut s;
    let r2 = &mut s;
    
    println!("{}, {}", r1, r2); // Error!
    ```
    
    - 위와 같이 borrowing한 값을 사용하기 전에 또 borrowing을 하면 컴파일 에러가 난다.
    - 그래서 scope가 분리되어 있으면 여러 개의 mutable reference를 사용할 수 있다.
        
        ```rust
        let mut s = String::from("hello");
        
        {
            let r1 = &mut s;
        } // r1 goes out of scope here, so we can make a new reference with no problems.
        
        let r2 = &mut s;
        ```
        
- 이런 제약 조건은 data racing이 일어나지 않게 해주는 장점이 있다.
- mutable reference와 immutable reference를 같이 사용 할 수 없다.
- 만약 아래와 같이 코드를 작성하면 같이 사용 할 수 있다.
    
    ```rust
    let mut s = String::from("hello");
    
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point
    
    let r3 = &mut s; // no problem
    println!("{}", r3);
    ```
    
    - reference가 scope가 끝날 때 까지 사용 되지 않는 것을 컴파일러가 말해준다. 이것을 [Non-lexical-lifetime(NLL)](https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html#non-lexical-lifetimes)이라고 한다.

### Dangling References

- point를 사용하는 언어에서는 dangling pointer를 만들 수 있는 위험이 있다. dangling pointer는 A 값을 가르키는 reference가 메모리에 있는데 A 값이 메모리에서 반납되는 경우를 말한다.
- 하지만 러스트에서는 이것을 컴파일 타임 때 잡아내서 에러를 출력한다.
    
    ```rust
    fn main() {
        let reference_to_nothing = dangle();
    }
    
    fn dangle() -> &String {
        let s = String::from("hello");
    
        &s
    } // 함수가 끝났으므로 변수 s는 메모리로부터 사라진다. &s는 dangling pointer가 된다.
    ```

## [The Slice Type](https://doc.rust-lang.org/book/ch04-03-slices.html)

- slice는 reference라서, ownership을 가지지 않는다.
- string slice 아래와 같이 한다.
    
    ```rust
    let s = String::from("hello world");
    
    let hello = &s[0..5]; // python과 똑같이 5번째 인덱스는 포함 X. &s[..5]로 쓸 수도 있다.
    let world = &s[6..11]; // 11이 끝이면 &s[6..]와 같이 쓸 수도 있다.
    ```
    각 변수가 가르키는 모습은 아래와 같다
    ![Untitled](https://doc.rust-lang.org/book/img/trpl04-06.svg)
    
- `&str` 는 string slice의 타입이다.
- `&String` 과 `&str` 은 같이 쓸 수 있다. 왜냐하면, String의 reference는 String의 전체 부분을 slice 한거랑 같기 때문이다.
- string literal(고정 문자열)은 그 자체가 `&str` 타입이다.
- array 슬라이스도 비슷하다.
    
    ```rust
    let a = [1, 2, 3, 4, 5];
    
    let slice = &a[1..3]; // slice는 &[i32] 타입이다.
    
    assert_eq!(slice, &[2, 3]);
    ```
    

# [Using Structs to Structure Related Data](https://doc.rust-lang.org/book/ch05-00-structs.html#using-structs-to-structure-related-data)

## [Defining and Instantiating Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)

```rust
struct User {
		active: bool,
		username: String,
		email: String,
		sign_in_count: u64,
}

// 초기화
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

// 필드에 접근
user1.email

// 할당
let mut user1 = User...
user1.email = String::from("anotheremail@example.com");

// Init shorthand
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

이미 초기화 된 구조체를 가지고 새로운 구조체를 만들 수도 있다.

```rust
let user2 = User {
    email: String::from("another@example.com"),
    ..user1
}; // javascript 문법이랑 비슷하다.
```

이 문법을 쓸 때 주의할 점이 있다. 이 문법을 사용하는 것은 `=` 를 사용해서 할당을 하는 것과 같다. 그래서 **user1의 username의 데이터가 이동하게 되어, user1 변수는 유효하지 않게 된다.**

만약 user1의 active와 sign_in_count 변수만 사용 했다면 [이 값들은 스택에 있는 값들이고 Copy 특성을 구현하기 때문에 user1 변수](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#stack-only-data-copy)는 계속 유효하다.

### Tuple structs

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

- struct와 다르게 필드 이름이 없다.
- black과 origin은 서로 다른 타입이다.
- tuple structs는 tuple 같이 행동해서, destructure 도 할 수 있다.

### Unit-like Structs

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

- 필드를 선언하지 않는 구조체이다.
- 특정 유형에 대한 특성을 구현해야 하지만 유형 자체에 저장하려는 데이터가 없을 때 유용하다.

### Ownership of Struct Data

- 위의 예제에서 string slice type인 `&str` 이 아닌 `String`을 썼다. 이 이유는 각각의 인스턴스가 스스로의 데이터를 갖고 해당 데이터는 구조체가 유효할 때까지 유효함을 유지시키기 위한 선택이다.
- 물론 구조체가 데이터에 대한 reference를 저장하게 할 수 있다. 대신 이럴려면 lifetimes라는 개념을 사용해야 하는데, 이 개념은 챕터 10에서 배운다.
- Lifetimes는 구조체에 저장 된 reference가 구조체가 유효할 때까지 유효함을 보장하는 개념이다.

## **An Example Program Using Structs**

- 일반 원시 타입은 println! 매크로로 출력 할 때 `{}` 문법을 쓰면 되는데, 구조체는 복잡하기 때문에 해당 문법이 아닌 `{:?}`(또는 `{:#?}` ⇒ pretty formatter) 문법을 사용 해야한다.
    
    ```rust
    #[derive(Debug)]
    struct Rectangle {
    }
    
    let rect = Rectangle {...}
    
    println!("rect : {:?}", rect)
    ```
    
    - 구조체를 특정 기능에 사용하고 싶으면 위와 같이 `#[derive(Debug)]` 를 작성해서 명시 해줘야 한다.
- 디버깅하기 위해 출력하는 매크로 중에 println!보다 효과적인 dbg! 매크로가 있다.
    
    ```rust
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    fn main() {
        let scale = 2;
        let rect1 = Rectangle {
            width: dbg!(30 * scale), // ownership을 반환해주기 때문에 이렇게 할당 가능
            height: 50,
        };
    
        dbg!(&rect1);
    }
    ```
    
    - dbg!는 표현식의 ownership을 가지고 간 뒤에 반환 해준다.
    - dbg!는 파일명과 라인 번호 그리고 표현식의 결과를 반환해준다.
        
        ```rust
        // [src/main.rs:10] 30 * scale = 60
        ```
        

### Method Syntax

- 메소드는 impl 키워드를 써서 정의한다.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
		// &self는 self: &Self의 축약어
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

- 위의 메소드의 인자에서 확인 할 수 있듯이, 메소드는 ownership을 가지고 오지 않고 빌리는 형태가 일반적이다.
- ownership 자체를 가지고 오는(&self가 아닌 self를 씀) 경우는 드물며, self를 다른 형태로 변형하거나 호출 된 곳에서 원래의 인스턴스를 사용하는 것을 막고 싶을 때 아니면 해당 테크닉은 잘 사용하지 않는다.
- C나 C++에서는 객체의 pointer로 메서드를 호출하는 분법이 있다: `object-something()`
- 대신에, Rust는 자동 참조와 역참조(automati referencing and dereferencing)이라는 특징을 가지고 있다.
- Rust는 정의 된 메서드의 self가 읽기용(&self)인지, 수정 가능한지(&mut self), 소비하는 것인지(self)를 읽고 자동으로 `&`, `&mut`, `*` 표현식을 붙혀준다. 그래서 아래의 코드는 서로 같다.
    
    ```rust
    p1.distance(&p2);
    (&p1).distance(&p2);
    ```
    
- impl block에 정의 되면서 self를 인자로 갖지 않는 함수를 associated function이라고 한다. 보통 constructor의 역할을 하는 함수를 만들 때 사용한다.
    
    ```rust
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    impl Rectangle {
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }
    
    fn main() {
        let sq = Rectangle::square(3);
    }
    ```
    
- impl block은 여러 개 정의 할 수 있다.

# **[Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html#enums-and-pattern-matching)**

## Defining an Enum

- 구조체를 사용해서 커스텀 데이터 타입을 정의하는 방법
    
    ```rust
    enum IpAddrKind {
    		V4,
    		V6,
    }
    
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    fn route(ip_kind: IpAddrKind) {}
    
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    
    struct IpAddr {
    		kind: IpAddrKind,
    		address: String,
    }
    
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    ```
    
- enum의 값에 데이터를 직접 넣을 수 있다.
    
    ```rust
    enum IpAddr {
        V4(String),
        V6(String),
    }
    
    let home = IpAddr::V4(String::from("127.0.0.1"));
    
    let loopback = IpAddr::V6(String::from("::1"));
    ```
    
    - 이렇게 타입을 정의하면, V4와 V6는 IpAddr 인스턴스를 만드는 constructor 함수가 된다.
- enum은 서로 다른 타입의 변수를 가질 수 있다.
    
    ```rust
    enum IpAddr {
    		V4(u8, u8, u8, u8),
    		V6(String),
    }
    
    let home = IpAddr::V4(127, 0, 0, 1);
    
    let loopback = IpAddr::V6(String::from("::1"));
    ```
    
- enum은 구조체와 같이 impl 키워드를 사용해서 메서드를 정의 할 수 있다.
    
    ```rust
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    
    impl Message {
    	  fn call(&self) {
    	      // method body would be defined here
    	  }
    }
    
    let m = Message::Write(String::from("hello"));
    m.call();
    ```