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

# [Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)

- Rust에는 scalar와 compound라는 두 가지 데이터 타입이 있다.
- Rust는 정적 타입 언어이다. 즉 컴파일 타임에 모든 변수는 타입이 정해져 있어야 한다는 의미다.
- 개발자가``parse` 같은 메소드로 타입 변환을 한다면 반드시 타입 명시(annotation)를 해줘야 한다. 만약 하지 않으면 컴파일 타임 때 에러가 난다.

## Scalar Types

- 하나의 값을 표현하는 타입
- 정수, 소수, Boolean, character

### Integer Types

- `i` : 부호가 있는 정수, `u` : 부호가 없는 정수
- 1_000 과 1000은 같은 숫자. 앞은 읽기 쉬운 표기법
- rust의 기본 정수 타입은 i32
- usize, isize 타입은 컴퓨터 아키텍처에 따라서 크키가 정해짐. 64 비트 아키텍처의 컴퓨터이면 64 비트 타입이 됨.
- **정수 오버플로우**
    - 디버그 모드에서 컴파일 시 정수 오버플로우가 생기면 panic이 발생함
    - 릴리스 모드에서 컴파일 시(`—release` 플래그 사용)  정수 오버플로우를 체크하지 않음. 오버플로우가 발생하면 panic 시키지 않음. 대신 타입의 최대값보다 큰 값만큼 “wrap around”라는 처리를 함. 예를 들어 u8 타입에서 256은 0으로, 257은 1로 변환함. *(번역 부족)*
    - 이를 핸들링하기 위해서 스탠다드 라이브러리에서 제공해주는 오버 플로우 관련 메소드들을 사용하면 됨. ([예시](https://doc.rust-lang.org/std/?search=overflowing))

### Floating-Point Types

- f32, f64 두 가지 타입이 있다.
- f64 타입은 현재 CPU로 같은 속도와 높은 정확성을 가지고 있기 때문에 기본 소수 타입으로 사용 된다.

### The Boolean Type

```rust
fn main() {
	let t = true;
	let f: bool = false;
}
```

### The Character Type

```rust
fn main() {
	let c = 'z'
}
```

- single quote로 선언 해야 한다. double quote를 사용하면 string 타입이 된다.
- rust의 char 타입은 Unicode Scalar Value를 표현 할 수 있는 4바이트의 크기를 가진다.

## Compound Types

- 다수의 값을 가짐
- tuple과 array 타입이 있다.

### The Tuple Type

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
    

### The Array Type

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