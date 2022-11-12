/*
    trait는 다른 언어의 인터페이스라고 부르는 기능들과 유사하지만 약간 차이가 있음

    Rust가 다형성을 제공하는 방법은 2가지 -> 정적 디스패치(static dispatch)와 동적 디스패치(dynamic dispatch)

    - 정적 디스패치는 "컴파일 타임"에 어느 함수에 호출이 바인딩 되는지 결정
    - 동적 디스패치는 "런타임"에 어느 함수에 호출이 바인딩 되는지 결정

    정적 디스패치를 사용하면 컴파일 시 모든 코드들을 생성하고 컴파일 하게 되므로 컴파일 시간이 길어지고, 바이너리의 크기가 커짐
    그리고, CPU 명령어 캐시 최적화에도 방해가 됨(?)

    동적 디스패치
*/

#[derive(Clone)]
struct Product {
    name: String,
    price: u64,
    quantity: u64,
    production_date: u64,
}

#[derive(Clone)]
struct Employee {
    name: String,
    id: u64,
}

// Java의 interface와 유사함
// 공유 메서드
trait PrintInfo {
    fn print_info(&self);
}

impl PrintInfo for Employee {
    fn print_info(&self) {
        println!(
            "employee's name: {}\nemployee's id: {}\n",
            self.name(),
            self.id()
        );
    }
}

impl PrintInfo for Product {
    fn print_info(&self) {
        println!(
            "Product's name: {}\nProduct's price: {}\nProduct quantity: {}\nProduction date: {}",
            self.name,
            self.price,
            self.quantity,
            self.date()
        );
    }
}

impl Product {
    fn new(name: String, price: u64, quantity: u64, production_date: u64) -> Product {
        Product {
            name,
            price,
            quantity,
            production_date,
        }
    }

    fn date(&self) -> String {
        let mut date = self.production_date;
        let year: u64 = date / 10000;
        date = date - year * 10000;
        let month = date / 100;
        date = date - month * 100;
        format!("{}/{}/{}", year, month, date)
    }
}

impl Employee {
    fn new_from_default() -> Employee {
        Employee {
            name: "default".to_string(),
            id: 100,
        }
    }

    fn new(name: String, id: u64) -> Employee {
        Employee { name, id }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn id(&self) -> u64 {
        self.id
    }
}

// complie time에 어떤 데이터가 들어오는지 하드 코딩 됨
fn static_print(data: impl PrintInfo) {
    data.print_info();
}

// 런타임에 어떤 데이터가 들어오는 알 수 있음
// dyn을 사용하여 dynamic dispatch임을 알려주고 Box<T>를 사용해서 컴파일러가
// 메모리의 크기가 불확정하다고 판단하지 않게 하기 위해 힙 메모리에 넣어줌
fn dynamic_print(data: Box<dyn PrintInfo>) {
    data.print_info();
}

fn main() {
    let employee = Employee::new("Jane".to_string(), 100);
    let product = Product::new("Apple".to_string(), 1, 100, 20220924);

    static_print(employee.clone());
    static_print(product.clone());
    dynamic_print(Box::new(employee));
    dynamic_print(Box::new(product));
}
