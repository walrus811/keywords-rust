From [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html).

-   ### [도구](#tool)

-   ### [문법](#syntax)

<a id="tool"></a>

## 도구

### rustup

-   Rust 도구

```
$ rustup update
$ rustup self uninstall
$ rustup doc
```

### rustc

-   컴파일러
-   AOT 컴파일
-   언어 구조상 컴파일 타임에 레퍼런스나 동시성 문제를 잡아낼 수 있음

```
$ rustc --version
```

### cargo

-   빌드 시스템이자 패키지 매니저.
-   시맨틱 버전 사용.
-   [crates.io](https://crates.io/)
-   실행 커맨드 동작 시 소스의 변화를 감지하여 필요한 순간에만 빌드한다.

```
$ cargo --version
$ cargo new [프로젝트 이름] [--lib]
$ cargo build [--release]
$ cargo run # 빌드와 실행을 하는 올인원 커맨드
$ cargo check # 코드가 컴파일 될 수 있는지 빠르게 체크하지만, 결과물은 만들지 않는다.
$ cargo update # 프로젝트 내 crate 버전 업데이트, lock 파일을 무시하고 'cago.toml' 내 버전으로 lock 파일을 최신화한다.
$ cargo doc [--open] # 프로젝트 내 포함된 의존성 문서를 만든다.
$ cargo [커맨드] -- [cargo가 아닌 프로그램의 아규먼트]
```

#### cargo 프로젝트 구조

-   소스 코드는 `src`에 보관
-   루트 디렉토리에는 README, 라이센스, 설정 파일 등 보관

```
> tree
[프로젝트 이름]
│  Cargo.toml
│  Cargo.lock
└─src
│       main.rs
└─target
        [cargo build 결과물]
```

### toml

-   컨피그 파일 형식(https://toml.io/en/)

### 컨벤션

-   소스 파일 이름은 `snake_case`로 짓는다.

### Crates.io

-   [패키지 저장소](https://crates.io/)

<a id="syntax"></a>

## 문법

### 매크로

-   이름 뒤에 `!`가 붙음

```rust
println!("Hello world!");
eprintln!("Hello world!");
dbg!("dgggg");
```

### indent

-   탭이 아니라 스페이스 4개를 씀

### prelude

-   표준 라이브러리 중 Rust가 자동으로 임포트하는 것들
-   굳이 임포트할 필요는 없다.
-   [목록](https://doc.rust-lang.org/std/prelude/index.html)

### 변수

-   `let` 키워드로 변수를 선언한다. 기본적으로 **불변**. 컴파일 타임에 안전한 코드를 작성할 수 있게 해준다.
-   `mut`, 이름 앞에 사용, 사용시 가변. 명시적인 사용으로 이 변수를 다른 코드가 바꿀 수 있다는 걸 암시한다.
-   컨벤션으로 `snake_case`를 사용한다.

```rust
let apples = 5;
let mut banana= 10;
```

### 상수

-   `const` 키워드를 쓴다.
-   타입이 항상 명시 되어야한다.
-   스코프 상관 없이 쓰일 수 있다.
-   컴파일 타임에 값이 결정가능한 표현식만 온다.
-   자신이 속한 스코프 내에서 프로그램이 끝날 때까지 유효하다.
-   `mut`와 쓸 수 없다. 상수는 변경 될 수 있다 개념이 없기 때문이다.
-   컨벤션으로 `UPPER_CASE`를 사용한다.

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

### 변수의 섀도잉

-   같은 스코프 내에서 동일한 변수를 선언할 경우, 이전 변수가 가려짐.
-   `let`과 함께 동일한 이름을 쓴다.
-   변수를 다른 타입으로 바꾸는 등의 응용에 사용.
-   굳이 이름을 안 바꿔도 된다.
-   `mut`와는 여러 차이점이 존재한다.
    -   `let` 키워드를 사용하는 덕에 다른 스코프에서 해당 변수를 사용하고 변경하고도 변수의 불변성을 유지할 수 있다(컴파일 타임 에러를 얻을 수 있다).
    -   섀도잉은 변수의 타입을 변경하는 응용이 가능하다.

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // 12
    }
    println!("The value of x is: {x}"); // 6
}
```

```rust
fn main() {
    let spaces = "   ";

    {
        let spaces = spaces.len();
        println!("spaces as number: {spaces}"); // 4
    }

    println!("spaces as string: {spaces}"); //
}
```

### 데이터 타입

-   Rust는 정적 타입 언어다.
-   스칼라(Scalar) 타입과 컴파운드(Compound) 타입이 존재한다.
-   컴파일러는 알아서 타입을 유추해주기도 하지만, 여러 가능성이 있어서 추론이 불가능할 때는 반드시 타입 어노테이션을 붙여주어야 한다.

#### 스칼라 타입

-   단일 값을 나타낸다.

1. 정수(integer)

-   `i8`, `u8`, `i16`, `u16`, `i32`, `u32`, `i64`, `u64`, `i128`, `u128`, `isize`, `usize`가 존재한다.
-   `isize`와 `usize`는 머신의 레지스터 사이즈에 의존한다.
-   디폴트는 `i32`.
-   `_`은 숫자 리터럴을 읽기 쉽게 구분하는데 쓰임(`98_222`).
-   16진수(`0x`), 8진수(`0o`), 2진수(`0b`).
-   바이트(`b'A'`) - **u8 타입**
-   오버플로우시에 다음 두 동작 중 한가지가 벌어진다.
    -   디버그 모드시, Rust는 런타임에서 오버플로우 발생시 프로그램을 `panic` 상태로 만드는 체크 로직을 포함한다.
    -   릴리즈 모드시, Rust는 위 같은 체크로직을 포함하지 않는다. 대신 `2의 보수 래핑`이 벌어진다. 이 동작은 정수의 범위 내에서 값이 회전하는 동작을 의미한다. 가령, u8에서 256은 0이 되고, 257은 1이 되는 식이다. 즉, 프로그램이 `panic`에 빠지지 않는다.
-   명시적으로 오버플로우를 다루기 위해 `wrapping_*(래핑)`, `checked_*(None 값 리턴)`, `overflowing_*(값과 함께 boolean 값 리턴)`, `saturating_*(min~max로 coerce)` 메서드들을 활용한다.

> `panic`은 프로그램이 에러와 함께 종료 되었을 때를 일컫는다.

2. 실수(flating-point)

-   `f32`,`f64`가 존재한다.
-   디폴트는 `f64`.
-   리터럴을 사용할 때 `f32`는 타입을 명시해주어야 한다.

3. 불린(boolean)

-   `bool` 키워드를 사용한다.
-   `true`,`false` 값이 존재한다.

4. 문자(character)

-   `char` 키워드를 사용한다.
-   유니코드를 표현하기 위해 4바이트 크기를 가진다.
-   `'`를 사용한다.

#### 컴파운드 타입

-   한 타입에 여러 값이 들어가 있는 타입이다.

1. 튜플(Tuple)

-   이종의 데이터를 모아놓은 컴파운드 타입.
-   고정 크기를 가진다.
-   패턴 매칭으로 `destructuring` 동작이 가능하다.
-   `.` 뒤에 인덱스를 붙여 단일 값을 조회할 수도 있다.
-   아무런 값도 지니지 않은 특수한 튜플을 유닛(unit)이라고 한다. 값과 타입 그 자체가 `()`로 표현된다. 빈 값이나 빈 리턴 타입을 표현할 때 사용 된다.
-   표현식이 아무것도 리턴하지 않으면 유닛이 리턴 된다.

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;
let five_hundred = x.0;
let six_point_four = x.1;
let one = x.2;
```

2. 배열(Array)

-   동종의 데이터를 모아놓은 컴파운드 타입.
-   고정 길이.
-   데이터를 힙이 아니라 스택에 할당하고 싶을 때 유용하다.
-   동적 배열로는 표준 라이브러리에 `vector`가 있다.
-   같은 값을 여러번 할당하고 싶을 땐 `[값; 갯수]` 표현식을 사용한다.
-   인덱스를 벗어나는 접근은 `panic`을 발생시킨다. 이는 릴리즈 모드에서도 동일하다. 이는 메모리 오염 문제를 막기 위한 rust의 메모리 관리 원칙이다.

```rust
let a = [1, 2, 3, 4, 5];
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
let a: [i32; 5] = [1, 2, 3, 4, 5];
let a = [3; 5];
let a = [3, 3, 3, 3, 3];
```

```rust
fn main() {
    let sda = [3; 999999999999];
    let c = sda[0];
    println!("d {c}");
}
//zsh: segmentation fault  cargo run
//stack overflow?
```

### 함수

-   `fn` 키워드를 사용해 정의한다.
-   `main`은 특수한 함수로 프로그램 엔트리 포인트로 쓰인다.
-   컨벤션으로 `snake_case`를 사용한다.
-   함수 정의 순서나 위치는 신경 쓰지 않아도 된다.
-   매크로를 호출하는 것은 표현식이다.
-   반드시 파라미터 타입을 명시해야한다.
-   리턴 타입은 `->`로 표시한다.
-   Rust에서 함수의 리턴 값은 **블록의 마지막 표현식의 값**이다.
-   물론 `return` 키워드도 존재하지만 이는 앞서서 함수를 종료시킬 때 사용한다.
-   대부분의 경우, 마지막 표현식을 암시적인 리턴값으로 사용한다.
-   rust는 표현식 기반의 언어다.
-   마지막 표현식에는 `;`이 붙지 않는다.
-   아무것도 리턴하지 않으면 `()`, 유닛이 리턴 된다.
-   마지막 표현식에 `;`가 붙으면 아무것도 리턴하지 않는 게 되어버린다.

```rust
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```

```rust
let mut x = 1; // x - i32
let y = x = 12; // y - (), 즉, rust에서 할당은 표현식으로 평가 되지 않는다.
```

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1 // ;이 없다. 이 표현식이 평가 된다.
        // x+1; 과 같이 ;를 넣는다. 이러면 표현식이 아니라 구문이 되어버리므로 y는 유닛()이 된다.
    };
    //4!
    println!("The value of y is: {y}");
}
```

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
```

### 주석

-   `//`만 쓴다.

### if 표현식

-   `if` 표현식이 조건이 맞을 때 실행하는 코드를 `arm`이라고도 한다. 이는 `match` 표현식과 동일하다.
-   조건은 반드신 `bool`이어야만 한다(`if 3` 같은 건 안 된다).
-   `else if`가 두 개가 넘어간다면 `match`를 고려해 볼 것.
-   `if`는 표현식이므로 `let`과도 같이 쓰일 수 있다. 이 경우, 모든 표현식이 같은 타입으로 평가 되어야 한다.

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
```

### 루프

-   `loop`,`while`,`for`가 존재한다.
-   `loop`는 명시적인 중단이 있을 때까지 계속 반복한다.
-   `loop`는 `break`를 통해 값을 리턴할 수도 있다.
-   `loop`에 레이블을 붙일 수도 있다. 이를 통해 중첩된 구조의 빠른 탈출이 가능하다. 레이블 앞에는 `'`를 붙인다.
-   특정 인덱스나 갯수만큼 반복하는 것도 `Range`와 `for`를 활용하는 게 좋다.

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```

```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
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
    println!("End count = {count}");
}
```

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```

### 소유권(ownership)

-   rust가 메모리를 관리하는 규칙.
-   컴파일 단계에서 체크하므로 이 규칙으로 인해 런타임 성능 저하는 없다.
-   다음의 규칙을 갖는다.
    -   rust의 모든 값은 소유자(owner)를 갖는다.
    -   오직 하나의 소유자만 존재할 수 있다.
    -   소유자가 스코프에서 사라지면 값도 사라진다.
-   stack 저장되는 변수는 그 순서가 보존 되고 크기를 알 수 있으므로 일반적인 변수/스코프 규칙과 다를 게 없다. 스코프가 끝나면 변수에 할당된 메모리가 해제 된다.
-   static area에 저장되는 변수도 다를 것 없다. 문자열 리터럴(`str`)이 대표적인 예시다. 같이 컴파일 시 바이너리에 그 정보가 저장 되어 컴파일 타임에 크기를 알 수 있다. 프로그램이 끝나면 변수에 할당된 메모리가 해제 된다.
-   heap에 저장되는 변수가 문제다. 컴파일 타임에 크기를 알 수 없다. 그러므로 런타임에 heap에 메모리를 할당한다. 화두는 이 변수를 다 사용했을 때 어떻게 해당 메모리 공간을 정리할 것이냐다.
    -   GC(Garbage Collection) 메커니즘을 가진 언어는 메모리를 감시하여 더이상 쓰지 않는 메모리 공간을 알아서 해제 하므로 문제될 것 없다.
    -   C++처럼 명시적으로 메모리를 해제해주어야 하는 경우, 코딩 시 많은 주의를 기울여야 한다(dangling pointer, double free 등).
    -   rust는 **메모리 공간을 소유한 변수가 스코프를 벗어나면 바로 메모리를 자동으로 해제한다**.

```rust
    {                      // s는 유효하지 않음
        let s = "hello";   // s는 이제부터 유효

        // s로 무언가 한다.
    }                      // s가 스코프를 벗어났으므로 `drop`이 자동으로 호출 되어 메모리가 정리 된다.
```

#### Move

```rust
    let x = 5;
    let y = x;
```

-   문제 될 것 없다. `i32`는 4바이트로 그 크기가 정해져있고 stack에 할당되는 변수 타입이다. 자연스럽게 복사 되고 변수 `x`와 `y`가 생긴다.

```rust
    let s1 = String::from("hello");
    // s1 -> "hello"
    let s2 = s1;
    // s2 -> "hello"
    // s1(비활성화!)

    println!("{}, world!", s1);
    //error[E0382]: borrow of moved value: `s1`
```

-   문제는 위와 같은 경우인데 이 경우 `deep copy`는 물론, `shallow copy`조차 벌어지지 않는다. heap에 할당된 "hello"의 소유자가 s2가 되고, s1은 비활성화 된다. 그래서 위 코드는 컴파일 되지 않는다.
-   이런 동작을 `Move`라고 한다.
-   `deep copy`도 되지 않으니 따로 더 신경 쓸 것도 없고, `shallow copy`도 되지 않으니 `double free` 같은 문제로부터도 자유롭다.
-   성능상의 이점도 보장 되고 중요한 건 컴파일 타임에 이미 이에 대한 검증이 끝난다는 것이다.
-   이 동작은 `Copy` 트레이트를 구현하지 않았을 때 기본 동작이다. 정수 타입처럼 stack에 저장되는 타입에 대해 구현이 되어 있는 트레이트이다. 이 트레이트가 구현된 타입의 변수는 move를 하지 않고, stack 내에 deep copy를 한다.
-   rust는 타입 그 스스로나 그 일부가 `Drop` 트레이트를 구현한 경우, `Copy`를 쓸 수 없다. 이 경우도 컴파일 에러가 뜬다.
-   `Copy` 트레이트를 구현한 타입은 보통 스칼라 값이다. 외에도 메모리 할당을 하지 않는 타입이라면 가능하다. 가령, 스칼라 값으로 이루어진 튜플을 `Copy`를 구현할 수 있다.

```rust
    //deep copy
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
```

-   이런 동작은 함수 호출 시 파라미터에도 동일하게 적용 된다.

```rust
fn main() {
    let s = String::from("hello");

    takes_ownership(s);
    //이미 s는 소유권을 take_ownership의 some_string 파라미터로 넘겨주고 비활성화 되었다.
    println!("{}", s);
    //value borrowed here after move
    let x = 5;

    makes_copy(x);

    println!("{}", x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

```

```rust
fn main() {
    let tuple1 = (1, "test");

    let tuple2 = tuple1; // 튜플 값이 전부 스칼라이고 Copy 트레이트가 구현 되어있으니 여기서는 Move가 아니라 Copy가 이루어진다.

    println!("{:?}", tuple1);
    //(1, "test")
    println!("{:?}", tuple2);
    //(1, "test")
}
```

```rust
fn main() {
    //이렇게 리턴으로 소유권을 Move 시킬 수 있다.
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
```

```rust
fn main() {
    let s1 = String::from("hello");
    //그래도 매번 이 건 좀 아닌 거 같다. 이래서 레퍼런스가 존재한다.
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
```

### 레퍼런스

-   `&`를 이용해 선언한다. 레퍼런스를 통해 소유권 이동 없이 변수를 사용할 수 있다. 레퍼런스가 소유권을 가지지 않으므로 레퍼런스가 없어진다고 해서 값이 사라지지 않는다.
-   파라미터에 레퍼런스를 쓸지말지에 대해서는 다음과 같이 생각해보자. `이 함수가 해당 변수의 소유권을 가져야할 이유가 있는가?`
-   이렇게 레퍼런스를 만드는 동작을 `borrowing`이라고 한다.
-   레퍼런스 또한 기본적으로 불변이다.
-   `mut` 키워드로 가변으로 만들 수 있다. 당연히 **빌려주는 쪽**도 가변이어야 한다.

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    // &s -> s1 -> "hello"
    s.len()
}
```

-   가변 레퍼런스에는 중요한 제약이 존재한다.
    -   한 가변 변수에 대해 오직 단 하나의 가변 레퍼런스만 존재할 수 있다. 이를 통해 데이터 레이스 컨디션을 방지한다.
    -   불변 레퍼런스가 존재하는 한 가변 레퍼런스는 존재할 수 없다. 불변 레퍼런스를 쓴다는 건 해당 값이 변하지 않을 거라는 보증이나 마찬가지다. 가변 레퍼런스를 또 선언한다는 건 이 보증을 깨는 것이다. 마찬가지로 불변 레퍼런스가 존재하는데 원래 가변 변수로 값을 바꿀 수 없다.
-   중요한 건 이 모든게 문법 규칙으로 컴파일 타임에 잡을 수 있는 문제라는 것이다.

> 데이터 레이스 컨디션의 발생 조건
>
> -   둘 이상의 포인터가 같은 데이터에 동시에 접근
> -   적어도 한 포인터가 쓰기를 위해 사용 된다
> -   특정 데이터 접근에 대해 동기화 매커니즘이 없음

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;
            //^^^^^^ second mutable borrow occurs here
    println!("{}, {}", r1, r2);
}
```

```rust
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    }//이미 r1은 스코프를 벗어나 무효해졌으므로 이 코드는 동작한다.

    let r2 = &mut s;
```

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
          // -- immutable borrow occurs here
    let r2 = &s;
    let r3 = &mut s;
           // ^^^^^^ mutable borrow occurs here
    println!("{}, {}, and {}", r1, r2, r3);
}
```

```rust
fn main() {
    let mut i = 12;
    let i1 = &i;
    let i2 = &i;
    i = 123;
  //^^^^^^^ assignment to borrowed `i` occurs here
    println!("{} {}", i1, i2);
}
```

-   레퍼런스의 수명은 선언부터 **레퍼런스가 마지막으로 사용된 순간**까지다.
-   레퍼런스는 언제나 유효하도록 컴파일러가 보증한다. `dangling pointer` 같은 문제가 벌어지지 않는다.

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    //r1,r2의 사용은 여기서 끝났다. 그러므로 가변 레퍼런스를 쓸 수 있다.
    let r3 = &mut s;
    println!("{}", r3);
}
```

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
    //함수가 종료 되는 순간, s는 drop 되고, &s는 무효하다.
    //   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
    //help: consider using the `'static` lifetime
}
```

### 슬라이스 타입

-   컬렉션의 일부를 참조할 수 있는 슬라이스도 레퍼런스의 일종이다.
-   range 표현식의 `..`은 `[)`의 범위를, `..=`은 `[]`의 범위를 의미한다. 처음부터 시작하면 `[..2]` 같은 식으로 쓸 수 있다. 끝까지라면 `[3..]`처럼 쓸 수 있다. `[..]`은 컬렉션 전체를 나타낸다.
-   레퍼런스이기에 슬라이스를 쓸 때는 `&`가 필수다.

```rust
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
```

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear();
  //^^^^^^^^^ mutable borrow occurs here
    // word는 레퍼런스인데다가 아직 유효하다.

    println!("the first word is: {}", word);
}
```

-   문자열 리터럴 변수는 `&str` 타입인인데 이는 즉, 해당 변수가 바이너리 내 문자열을 가리키는 문자열 슬라이스기 때문이다. 슬라이스이기 때문에 불변이다.
-   다음의 코드는 바이너리에 "Hello"와 "hey", 두 개의 문자열이 저장되어있을 뿐이다.

```rust
    let mut s = "Hello";
    let s2 = s;

    s = "hey";
    println!("{}", s2);
```

-   파라미터로 `&String` 보다는 `&str` 타입을 쓰는 게 더욱 일반적이다. 전자는 리터럴은 받아들이지 못하기 때문이다. 후자는 `String`과 `&str` 전부 받아들일 수 있다.

-   물론 슬라이스는 문자열 외에 다른 컬렉션에도 사용할 수 있다.

```rust
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
```

### struct

-   대문자로 시작하는 네이밍 컨벤션을 쓴다.
-   어느 특정 필드만 가변으로 변경할 수 없다.

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}
```

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email, // js처럼 간략하게도 쓸 수 있다.
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

-   js의 오브젝트 스프레드와 유사한 `..`이 존재한다(update syntax).
-   다만 이 연산은 `=`로 인식 되어 `Copy` 트레이트가 구현 되어있지 않은 필드에 적용하면 필드의 소유권이 이동해 **해당 필드**가 무효화 되버리므로 주의해야한다. 이렇게 소유권이 이동한 채로 인스턴스 그 자체는 사용할 수 없지만 소유권을 가지고 있는 필드는 여전히 사용할 수 있다.
-   컴파일러는 이런 동작을 `partial move`라고 표현한다.

```rust
fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // update syntax
    };
}
```

```rust
fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    // user1의 username 소유권이 user2의 username으로 넘어가 user1이 무효화 된다.
    println!("{:?}", user1)
                    //^^^^^ value borrowed here after partial move
}
```

```rust
fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("another"),
        ..user1
    };
    // 나머지 필드는 `Copy` 트레이트가 구현 되어있으므로 소유권에 있어서 문제가 발생하지 않는다.
    println!("{:?}", user1)
}
```

```rust
fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    //active는 유효하다.
    println!("{:?}", user1.active)
}
```

#### tuple struct

-   struct를 마치 튜플처럼 사용할 수도 있다.
-   필드에 접근할 땐 `.0`처럼 튜플과 동일하다.

```rust
//둘은 다른 타입이다.
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

#### unit-like struct

-   유닛, ()처럼 동작하는 struct로 필드가 없다.
-   특정 타입에 트레이트를 구현할 거지만 그에 수반 되는 데이터가 없을 때 유용하다.

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

> `println!`과 `Debug` 트레이트
>
> -   struct는 `println!`의 `{}`로 출력 할 수 없다.
> -   이는 struct가 `std::fmt::Display`를 구현하지 않았기 때문이다.
> -   struct를 어떻게 보여줄지는 사람마다 다양한 의견이 있을 수 있기에 그런 것이다.
> -   이 경우, 컴파일러가 `{:?}`나 `{:#?}` 같은 플레이스 홀더를 권하는 걸 볼 수 있다. 후자는 줄도 바꾸는등 조금 더 이쁘게 보여준다.
> -   하지만 이 것도 사용하기 위해서는 struct가 `Debug` 트레이트를 구현해야한다.
>
> ```rust
> #[derive(Debug)]
> struct Rectangle {
>    width: u32,
>    height: u32,
> }
>
> fn main() {
>   let rect1 = Rectangle {
>       width: 30,
>       height: 50,
>   };
>    println!("rect1 is {:?}", rect1);
> // rect1 is Rectangle { width: 30, height: 50 }
> }
> ```
>
> -   위처럼 derive 어트리뷰트를 사용해 구현할 수 있다.

> `dbg!` 매크로
>
> -   이외에도 `dbg!` 매크로가 존재한다.
> -   주의할 점은 이 매크로는 `println!`과는 다르게 파라미터의 소유권을 가져간다는 것이다(레퍼런스를 쓰는 버전도 있다).
> -   그외에도 stderr로 출력된다는 점이 다르다.
> -   소스의 라인까지 출력된다.
>
> ```
> [src/main.rs:10] 30 * scale = 60
> [src/main.rs:14] &rect1 = Rectangle {
>     width: 60,
>     height: 50,
> }
> ```

#### 메서드

-   `impl` 블록 내에 정의한다. `impl`을 여러군데 사용할 수도 있다.
-   첫 파라미터는 `&self`로 나타내며, 이는 `self: &Self`를 나타낸다. 물론 레퍼런스를 안 쓸 수도 있지만 그 경우, 예외 없이 소유권이 넘어간다. 이런 경우는 굉장히 드문 상황이다. 보통 메서드가 self를 다른 걸로 바꾸고 호출한 측에서 원래 인스턴스를 못 쓰게 막는 경우에 쓰인다.
-   메서드의 이름은 필드와 동일할 수 있다. 메서드와 필드는 호출시 `()`로 구분할 수 있기 때문이다. 보통 `getter`를 정의할 때 쓰인다.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

#### associated function

-   `impl` 내에 정의된 모든 함수는 `associated function`이라고 부른다.
-   메서드는 이중 첫 파라미터로 `self`를 갖는 함수다.
-   메서드가 아닌 associated function은 `self`를 첫 파라미터로 갖지 않는다. `String::from`이 그 예시중 하나이며 보통 이렇게 새 인스턴스를 만드는 생성자에 쓰인다. 보통 `new`라고 부른다. 하지만 `new`는 언어의 키워드가 아니므로 이름에 제한은 없다.
-   `Self` 키워드는 `impl` 다음에 쓰여진 타입을 의미한다.
-   이런 함수를 호출하기 위해서는 `Rectangle::square(3)`처럼 `::`를 사용해야한다.

```rust
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```

### enum

-   기본적으로는 아래처럼 사용한다.
-   enum의 가능한 각 값을 `variant`라고 부른다.

```rust
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

fn route(ip_kind: IpAddrKind) {}
```

-   rust의 enum은 이 이상으로 특별하다. 바로 각 variant가 값을 가질 수 있다는 점이다.
-   심지어 각 variant의 타입이 같지 않아도 된다.

```rust
fn main() {
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
}
```

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

-   심지어 메서드도 추가할 수 있다.

```rust
fn main() {
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}
```

#### Result

-   `OK`와 `Err`의 값을 지니며 `expect` 메서드를 가지고 있음.
-   `expect`는 Result의 값이 `Err`일 때 인자로 건넨 메시지를 출력하고 프로그램을 죽임.

#### Option

-   rust는 null이 없다. 이는 프로그래머들이 은연중에 null을 `유효하지 않은 상태`로 생각하지 않고 하나의 값으로 여기는 문제에서 벗어나기 위한 언어 디자인적인 선택이다.
-   rust의 표준라이브러리는 이에 대해 `Option`이라는 enum을 제공한다. 이는 prelude에 포함 되어있으므로 따로 임포트하지 않아도 된다.
-   `None`은 null 본연의 의미인 상태(값)의 부재를 나타내고, `Some`은 상태가 존재함을 나타낸다.
-   타입 파라미터 자체인 `T`와 `Some<T>`는 다른 타입이므로 둘 사이에 연산은 불가능하다.
-   값을 다루기위해서는 `unwrap()`메서드도 있지만, 대부분의 경우 `match`를 이용한 패턴 매칭을 주로 쓴다.

```rust
#![allow(unused)]
enum Option<T> {
    None,
    Some(T),
}

fn main() {
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}
```

### match

-   패턴 매칭표현식
-   패턴은 리터럴, 변수 이름, 와일드 카드 등으로 이루어진다.
-   `_`은 catch-all 패턴이다. 디폴트 케이스를 정의할 때 쓰인다. 사실 마지막 패턴에 아무 이름이나 줘도 똑같이 동작한다.
-   패턴이 매칭 되어도 아무것도 하고 싶지 않을 때는 ()을 쓴다.
-   `arm`으로 이루어짐
-   `match` `[enumreation을 리턴하는 표현식]` {`arm 목록`}의 형식으로 이루어진다.
-   `if`와는 다르게 boolean이 아닌 패턴도 사용할 수 있다.
-   모든 패턴을 기술해야한다(exhaustive match).

```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
}
```

```rust
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
}
```

### if let

-   `match`에 대한 대용으로 `if let`도 존재한다. 보통 한 패턴만 유효할 때 간략한 표현을 위해 이 문법을 쓴다.
-   하지만 `macth`처럼 모든 경우를 체크하진 않는다.
-   그냥 syntatic sugar정도로 생각하면 된다.
-   else도 쓸 수 있는데 굳이?

```rust
fn main() {
    let config_max = Some(3u8);
    //  match config_max {
    //    Some(max) => println!("The maximum is configured to be {}", max),
    //    _ => (),
    //}
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}
```

### 프로젝트 관리(모듈 시스템)

#### 가이드라인

-   *lib.rs*에 기능을 넣고, 그 기능을 *main.rs*가 사용하는 형식으로 개발
-   세팅을 제외한 전체 로직은 *lib.rs*에 러너함수를 만들어서 쓴다.
-   *lib.rs*에 테스트 코드

#### 크레이트(crate)

-   컴파일러가 한 번에 인식할 수 있는 가장 작은 코드 단위
-   하나의 소스 파일을 컴파일러에 던져도 컴파일러는 해당 파일을 크레이트라고 인식한다.
-   `바이너리 크레이트`와 `라이브러리 크레이트`가 존재한다.
    -   바이너리 크레이트는 말 그대로 실행할 수 있는 바이너리 파일이다. `main`함수를 포함한다.
    -   라이브러리 크레이트는 다수의 프로젝트에서 공유하는 소스가 정의 되어있다. `main`함수를 포함하지 않으며 실행파일도 만들지 않는다. 일반적으로 크레이트는 이 라이브러리 크레이트를 의미한다. 라이브러리라고도 부른다.
-   크레이트의 루트는 컴파일러의 시작점이 되는 소스파일이며 이를 통해 크레이트의 루트 모듈을 생성한다.

#### 패키지(package)

-   하나 이상의 크레이트를 모아놓은 것을 패키지라고 한다.
-   `Cargo.toml` 파일을 포함한다. 이 파일은 크레이트를 어떻게 빌드할지에 대해 기술하고 있다.
-   `Cargo` 그 자체도 CLI 바이너리 크레이트를 포함한 패키지다. 이에 추가적으로 의존하는 라이브러리 크레이트도 포함하고 있다. 다른 프로젝트도 Cargo의 라이브러리 크레이트를 사용할 수 있다.
-   패키지는 적어도 하나의 크레이트를 포함해야 한다. 다수의 바이너리 크레이트를 포함할 수 있지만, 라이브러리 크레이트는 최대 한 개까지만 포함할 수 있다.
-   Cargo는 *src/main.rs*를 바이너리 크레이트의 루트로, *src/lib.rs*를 라이브러리 크레이트의 루트로 인식한다. 이 루트가 `rustc`로 건네져 컴파일이 시작 된다.
-   만약 패키지가 *src/main.rs*와 *src/lib.rs*를 동시에 가지고 있다면 바이너리와 라이브러리, 총 두 개의 크레이트를 가진 것이며 그 크레이트의 이름은 패키지와 동일하다.
-   패키지는 또한 *src/bin*에 다수의 바이너리 크레이트를 포함할 수 있다.

> 패키지가 바이너리 크레이트와 라이브러리 크레이트를 동시에 포함한 경우, 바이너리 크레이트가 마치 라이브러리 크레이트를 외부 패키지처럼 사용하는 방식으로 코딩을 할 것을 권장한다.

#### 모듈 시스템 요약

-   크레이트 루트부터 시작: 크레이트를 컴파일할 때 컴파일러는 일단 크레이트 루트(*src/main.rs*와 _src/lib.rs_)을 확인한다.
-   모듈 선언: 크레이트 루트 파일에 새 모듈을 선언할 수 있다. 가령, "garden" 모듈을 `mod garden;`으로 선언했다고 해보자. 컴파일러는 다음에서 모듈의 코드를 찾는다.
    -   인라인, `mod garden` 뒤의 세미콜론을 대체하는 중괄호 내
    -   _src/garden.rs_
    -   _src/garden/mod.rs_
-   서브모듈 선언: 크레이트 루트가 아닌 파일에서 서브모듈은 선언할 수 있다. 가령, `mod vegetables;`를 *src/garden.rs*에 선언한다고 해보자. 컴파일러는 서브모듈의 코드를 다음에서 찾는다.
    -   인라인, `mod vegetables;` 다음, 세미콜론 대신 중괄호 내
    -   _src/garden/vegetables.rs_
    -   _src/garden/vegetables/mod.rs_
-   모듈 내 패스: 모듈이 크레이트의 일부라면 모듈 내 코드를 크레이트 어디에서든 참조할 수 있다. 물론, 프라이버시 규칙을 따라야한다(private, publc). 가령, garden -> vegetables 모듈 내의 `Asparagus`는 `crate::garden::vegetables::Asparagus`로 찾을 수 있다.
-   private vs public: 모듈 내 코드는 기본적으로 부모에게는 private이다. 모듈을 public으로 만들기 위해서는 `pub mod`로 선언해야한다. public 모듈 내의 아이템을 public으로 만들기 위해서는 `pub`을 사용한다.
-   `use` 키워드: 스코프 내에서 `use` 키워드는 모듈의 긴 패스를 짧게 쓸수 있게 해준다. 가령, `use crate::garden::vegetables::Asparagus;`를 쓰면 스코프 내에서 `Asparagus`만으로 해당 아이템을 참조할 수 있다.

```
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

```rust
// src/marin.rs
use crate::garden::vegetables::Asparagus;

//컴파일러에게 src/garden.rs를 포함하라고 지시한다.
pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
```

```rust
// src/garden.rs
//컴파일러에게 src/garden/vegetables.rs를 포함하라고 지시한다.
pub mod vegetables;
```

```rust
// src/garden/vegetables.rs
#[derive(Debug)]
pub struct Asparagus {}
```

#### 모듈(module)

-   모듈 내 코드는 기본적으로 private.

```rust
// src/lib.rs
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

-   루트의 이름은 **crate**가 된다.
-   아래와 같은 모듈 트리가 그려진다.

```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment

```

#### 패스(path)

-   절대 경로는 crate부터 시작하며 상대 경로도 지원한다. 외에 `self`나 `super` 같은 키워드도 존재한다.
-   각 패스는 `::`로 구분된다.

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}
// 모든 자식은 그 부모에겐 private이다.
// 하지만 자식은 부모의 모든 게 public이다.
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
                         //^^^^^^^ private module

    // Relative path
    front_of_house::hosting::add_to_waitlist();
                    //^^^^^^^ private module
}
```

```rust
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
                                  //^^^^^^^^^^^^^^ private function


    // Relative path
    front_of_house::hosting::add_to_waitlist();
                           //^^^^^^^^^^^^^^ private function

}
```

```rust
//정상 동작
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

```rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
```

#### `pub`과 struct/enum

-   struct를 `pub`으로 선언해도 그 필드는 여전히 private이다. 노출하려면 따로 pub을 써주어야한다.
-   enum은 그 varaint들도 publc이 된다.

#### `use`

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

-   `use`는 특정 스코프에서만 동작한다. 그러므로 아래와 같은 코드는 유효하지 않다.

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

//use가 customer 안으로 들어가야한다.
use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
```

-   일반적인 용례는 함수인 경우는 위처럼 모듈까지만 쓰고, struct나 enum의 경우에는 절대 경로를 전부 사용한다.

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

-   `as`로 모듈에 새 이름을 줄 수도 있다.

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
    Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    Ok(())
}
```

#### `pub use`로 다시 익스포트 하기(re-exporting)

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

//front_of_houser::hosting을 외부로 다시 익스포트
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

#### 외부 패키지 사용하기

-   `Cargo.toml`의 의존성으로 작성하여 패키지를 포함한다.
-   그리고 `use`로 사용한다.
-   `std`도 마찬가지다.

#### 부모까지 패스가 같은 모듈

```rust
use std::cmp::Ordering;
use std::io;

//아래로 묶을 수 있다.
use std::{cmp::Ordering, io};
/////////////////////////////////////////
use std::io;
use std::io::Write;

//아래로 묶을 수 있다.
use std::io::{self, Write};
```

#### `glob` 연산자

-   주로 테스팅용으로 쓴다.

```rust
//std::collections 밑에 public 아이템을 다 가져온다.
use std::collections::*;
```

#### 파일로 분리

-   `mod` 선언은 모듈 트리 내에서 단 한번만 이루어진다. 그러니까 `mod`는 `include`가 아니다. 그러니까 C++에서 볼법한 인클루드 가드 같은 건 필요 없다.
-   `mod.rs`로 분리하는 법도 있긴한데 오래된 스타일로 요즘엔 안 쓰는 것 같다. 두 스타일을 같이 안 쓰는게 좋다. 그리고 `mod.rs`로 도배된 프로젝트라니 끔찍하다.

```rust
//src/lib.rs
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

//src/front_of_house.rs
pub mod hosting;

//src/front_of_house/hosting.rs
pub fn add_to_waitlist() {}
```

### 컬렉션

-   빌트인으로 제공 되는 컴파운드 타입(배열, 튜플)이 아니라 스탠다드 라이브러리가 제공하는 컬렉션.

#### Vec<T>

-   동적 배열.
-   `new`로 인스턴스를 만들지만, `vec!` 매크로를 써서 배열처럼 선언할 수도 있다.
-   인덱스나 `get` 메서드로 엘레멘트에 접근이 가능하다. 전자는 `&T`를 그대로 얻지만, 후자는 `Option<&T>`를 얻는다. 전자의 경우, 레퍼런스를 빼고 `T`를 얻으려할 경우, 소유권이 이동한다면 컴파일이 되지 않는다.
-   벡터 자체가 드랍될 때 안에 엘레멘트들도 드랍된다.

```rust
fn main() {
    //값을 넣지 않았기에 컴파일러가 추론할 수 없어 타입을 명시해주었다.
    //값이 이있으면 굳이 안 써도 된다.
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];
}
```

```rust
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
```

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}
```

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    //패닉에 빠진다.
    let does_not_exist = &v[100];
    //None이 된다.
    let does_not_exist = v.get(100);
}
```

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];
    // first 불변 레퍼런스가 유효하므로 에러가 난다.
    v.push(6);

    println!("The first element is: {first}");
}
```

```rust
    let v = vec![100, 32, 57];

    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        //가변 레퍼런스 수정을 위해서 '*' 연산자가 필요하다.
        *i += 50;
    }
```

```rust
fn main() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    //enum도 문제 없다.
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
```

#### String

-   코어의 `str`이 아닌, 스탠다드 라이브러리가 제공하는 동적인 문자열 타입.
-   UTF-8로 파싱
-   만드는데는 `new`, `to_string`, `from` 등의 방법이 있다.
-   업데이트할 때는 `pust_str`같은 메서드를 쓰거나, `+` 연산을 쓰거나, `format!` 매크로를 쓰는등 여러 방법이 있다.
-   `+` 연산자의 경우, 앞에 오퍼랜드의 경우 소유권을 취하고, 뒤의 오퍼랜드는 레퍼런스를 받는다. 그래서 앞의 변수는 무효화 된다. 전자의 소유권을 받아 후자를 추가한다음 전자의 소유권을 다시 리턴값으로 돌려주는 셈이다.
-   만일 여러 문자열을 이어붙일 때 `+`를 쓴다면 골치아플 수 있다. 이 때는 `format!` 매크로를 쓴다.
-   컴파일러는 `&String`을 `&str`로 만들어 버릴 수 있다. 이런 동작을 `deref coercion`이라고 한다.

```rust
    let mut s1 = String::new();

    let data = "initial contents";
    let s2 = data.to_string();
    let s3 = String::from("initial contents");
```

```rust
fn main() {
    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    //소유권을 취하지 않는다. &str을 받기 때문
    s1.push_str(s2);
    println!("s1 is {s1}");

    let mut s = String::from("lo");
    //단일 문자 추가
    s.push('l');
}
```

```rust
fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1의 소유권이 날아간다.
}
```

```rust
fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    //앞의 오퍼랜드의 소유권이 날아가므로 s1에만 영향을 미친다.
    let s = s1 + "-" + &s2 + "-" + &s3;
}
```

```rust
fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    //소유권은 유지된다.
    let s = format!("{s1}-{s2}-{s3}");
}
```

-   문자열은 엘레멘트에 대해 접근을 지원하지 않는다. 애초에 인덱스 자체가 구현이 되어있지 않다. 구현상 `String`은 `Vec<u8>`의 래퍼이지만, 문자열 자체가 `UTF-8`로 인코딩 되기 때문이다. 전체가 아스키로 표현 되어 1바이트로 표현 가능한 `String::from("Hola")`의 길이는 4이지만, 표현 범위가 1바이트를 넘어가는 `String::from("Здравствуйте")`의 경우, 그 길이가 24 바이트로 평가된다. 이 때문에 벡터처럼 인덱싱을 해도 정상적인 값을 얻어오지 못할 확률이 크다. 그렇기에 엘레멘트에 대한 인덱싱을 지원하지 않는 것이다.

> 키릴문자의 경우, 그 범위가 UTF-16 기준으로 0400-04FF이므로 2바이트로 표현 되는 것이다.

```rust
    //지스드라브스드부이데?
    let hello = "Здравствуйте";
    let answer = &hello[0]; //UTF-8 기준으로 0xD09E 전체가 아니라 0xD0만을 얻어올 것이다.
```

-   rust는 문자열을 세가지 관점에서 볼 수 있다. 바이트 모음, 스칼라 모음, 그리고 grapheme(문자의최소 단위) 모음이다. 이런 여러 관점이 있기에 인덱싱을 허용하지 않는다.

```rust
    let namaste = "नमस्ते";
    //as bytes(real data stored in computers)
    [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]

    //as char(unicode scalar)
    //이중 네번째랑, 여섯번째는 힌디 글자가 아니라 문법적인 요소란다...
    ['न', 'म', 'स', '्', 'त', 'े']

    //as grapheme clusters
    //문법적인 요소인 네번째랑 여섯번째가 병합 되었다.
    ["न", "म", "स्", "ते"]
```

-   인덱싱을 지원하지 않는 이유는 인덱싱이 위 같은 이유들로 인해 항상 `O(1)`의 성능을 보장할 수 없기 때문이다.
-   그러니 문자열 인덱싱은 포기하자.

```rust
fn main() {
    let hello = "Здравствуйте";

    let s = &hello[0..1];
    //thread 'main' panicked at 'byte index 1 is not a char boundary
    //친절하게 컴파일러가 잡아준다.
    print!("{}", s)
}
```

-   그래도 써야할 때는 조심히 슬라이스하거나 다음 방법을 쓴다. grapheme cluster는 스탠다드 라이브러리로는 지원 안한다. 화이팅.
    -   유니코드 스칼라: `chars()`
    -   바이트: `bytes()`

```rust
fn main() {
    for c in "피카츄".chars() {
        print!("{c} ");
    }
    println!("");
    for c in "피카츄".bytes() {
        print!("{c} ");
    }
}
//피 카 츄
//237 148 188 236 185 180 236 184 132
```

#### HashMap

-   해쉬맵. Vec이나 String과는 다르게 prelude에 포함이 안 되어있어 use로 임포트 해주어야 한다. 별도의 매크로도 없다.
-   `insert()`로 멤버를 추가하거나 업데이트 된다. 이미 키가 있으면 해당 키의 값이 업데이트 되니 주의해야한다.
-   `entry()`는 키가 없을 때 값을 추가할 수 있는 메서드다. 호출하면 `Entry` enum을 리턴한다. 이 enum은 값이 존재하는지 아닌지를 판단하는데 쓰인다.
-   해싱 함수로 SipHash라는 알고리즘을 쓴다. 가장 빠른 알고리즘은 아니지만, DoS 공격을 견딜 수 있는 알고리즘이라고 한다. 만약 빠른 알고리즘으로 교체하고 싶다면 별도의 `hasher`를 지정한다. 많은 알고리즘이 레포지토리에 있다고 한다.

```rust
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    //get은 Option<T>를 리턴한다.
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
```

```rust
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    //소유권이 넘어간다. field_name, field_value는 무효화 된다.
```

```rust
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    //25로 덧씌워진다.
    println!("{:?}", scores);
}
```

```rust
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}
```

```rust
fn main() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    //이런 활용도 가능하다.
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
```

### 에러 핸들링

-   rust의 에러는 복구 가능한 것과 복구 불가능한 것으로 나뉜다. 전자는 파일을 찾을 수 없는 등의 것이고, 후자는 배열의 범위를 벗어난 인덱스 참조처럼 프로그램을 즉각 종료해야하는 같은 심각한 것이다.
-   대부분의 언어가 예외라는 매커니즘으로 이를 구분하지 않는다. rust는 예외가 없다. 대신 복구 가능한 에러에 대해서는 `Result<T,E>`가, 복구 불가능한 에러에 대해서는 `panic!` 매크로가 존재한다.
-   추가적으로 backtrace라는 걸 확인할 수 있는데 조금더 자세하게 원인 내역을 볼 수 있다. 디버그 빌드에서만 동작하고, `RUST_BACKTRACE` 환경변수 값을 설정해야한다. 0, 1, 그리고 full의 값을 가질 수 있다.

#### 복구 불가능한 에러와 `panic!`

-   `panic!`이 호출 되는 경우는 다음 두 가지가 있다.
    -   배열 범위를 벗어난 인덱스를 참조하는등의 패닉을 유발하는 코드
    -   명시적으로 `panic!` 호출
-   패닉이 벌어지면 실패 메시지를 출력하고, 스택을 정리하고, 프로그램을 종료한다. 환경변수를 통해 콜스택 같은 정보를 확인할 수도 있다.

> 스택을 정리할 때 스택을 되감는(unwinding) 동작이 발생하는데 이는 굉장히 무거운 동작이다. 빌드할 때 이를 발생 시키지 않는 방법이 있다. 바로 `Cargo.toml`에 다음을 추가해주는 것이다.
>
> ```
> [profile.release]
> panic = 'abort'
> ```
>
> 이러면 패닉이 발생했을 때 즉각 프로그램이 종료 된다. 그렇게 되면 OS가 프로그램의 메모리를 정리해야하지만 바이너리의 크기가 줄어드는 효과도 있다.

```rust
fn main() {
    panic!("crash and burn");
}
/// thread 'main' panicked at 'crash and burn', src/main.rs:2:5
```

```rust
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
// thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
```

#### 복구 가능한 에러와 `Result`

-   파일을 읽으려는데 파일이 없는등 그렇게 심각하지 않은 에러.
-   `Result<T,E>` enum이 제공된다.
-   match를 써서 처리해도 되지만 `Result<T,E>`가 제공하는 메서드(`unwrap`,`expect`)를 사용하면 더 깔끔하다. 전자는 결과가 Ok인 경우,그 값을 리턴하고 아니면 `panic!`을 호출한다. 후자는 비슷하게 동작하지만 파라미터로 조금 더 상세한 메시지를 건넬 수 있다. 대부분 후자를 쓰는 게 더 낫다.

```rust
#![allow(unused)]
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}
```

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
}
```

#### 에러를 전달하기

-   한 함수에서 에러를 처리하지 않고 그 호출자에게 처리를 맡기는 패턴.
-   `?` 연산자를 사용하면 더 간편하게 해당 패턴을 구현할 수 있다. `Result`값 뒤에 오며 `match` 표현식과 거의 유사하게 동작한다. `Ok`일 때는 값을 그대로 리턴하고, 아니라면 그 즉시 `Err`값이 리턴되고 실행중인 함수는 종료 된다. 또 독특한 점은 `?`연산자에 의해 생긴 `Err`값은 `From` 트레이트의 `from()`메서드를 호출한다. 이 메서드를 통해 `Err`의 타입을 현재 함수의 리턴 타입으로 변경한다. 함수가 리턴할 수 있는 `Err` 타입이 하나일 때 유용하다고? 어차피 리턴 타입은 하나지.
-   리턴 타입이 명확하지 않으면 `match`, 하나의 `Result<T,E>`로 가능하면 `?`를 쓰는 게 낫다.

```rust
#![allow(unused)]
fn main() {
    use std::fs::File;
    use std::io::{self, Read};

    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username = String::new();

        File::open("hello.txt")?.read_to_string(&mut username)?;

        Ok(username)
    }

    fn read_username_from_file() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }
}
```

```rust
use std::fs::File;

fn main() {
    //main의 리턴타입은 ()이다.
    let greeting_file = File::open("hello.txt")?;
    //this function should return `Result` or `Option` to accept `?`
}
```

```rust
fn last_char_of_first_line(text: &str) -> Option<char> {
    // Option<T>도 가능하다.
    text.lines().next()?.chars().last()
}
```

```rust
use std::error::Error;
use std::fs::File;
// 메인의 리턴 타입도 수정할 수 있긴하다.
// Box<dyn Error>은 트레이트 오브젝트의 일종이며 어떤 에러가 될 수도 있다.
// main이 Result<(),E>를 리턴한다면 전통적인 방식대로 바이너리는 Ok라면 0을, Err이라면 0이 아닌 값을 리턴할 것이다.
// std::process::Termination 트레이트를 구현한 어떤 값이든 main은 리턴할 수 있다. 이 함수는 ExitCode를 리턴하는 report 함수를 포함한다.
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
```

### 제네릭(Generic)

-   제네릭이다. 아래 용례로 익히자.
-   컴파일 타임에 코드를 만든다(Monomorphization). c++하고 비슷한 짓을 한다.

```rust
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
//실패한다.
//help: consider restricting type parameter `T`
//binary operation `>` cannot be applied to type `&T`
//fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
// std::cmp::PartialOrd를 구현하지 않으면 비교할 수 없다.
```

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

enum Option<T> {
    Some(T),
    None,
}
```

```rust
struct Point<T> {
    x: T,
    y: T,
}

// 전체 타입에 적용
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 이렇게 타입별로 메서드를 정의할 수도 있다.
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

```rust
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

//이런 정신나간 응용도 가능하다.
impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

### 트레이트(Trait)

-   특정 타입이 가진 기능을 정의하고 다른 타입과 공유한다.
-   인터페이스와 유사하다.
-   `impl` 키워드로 타입에 대해 트레이트를 구현한다. 트레이트의 메서드를 전부 구현해야한다.
-   하지만 외부의 트레이트를 외부 타입에 구현할 순 없다. 가령, Vec<T>의 Display 트레이트를 내가 작성한 코드에는 구현할 수 없고, 그 역도 마찬가지다. 이런 제한들을 `coherence`라고 부르며, 좀 더 명확하게는 `orphan rule`이라고 한다. 왜냐하면 부모 타입을 직접 볼 수 없기 때문이다. 이를 통해 타인과 내 코드를 보호한다.
-   메서드에 대해 디폴트 구현도 할 수 있다. 구현시에 `impl`내에 해당 메서드 구현을 안하면 된다.

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    //디폴트 메서드가 다른 메서드를 호출하도록 할 수도 있다.
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```

#### 파라미터와 트레이트 바운드

-   함수 파라미터로도 쓸 수 있다. `&impl Summary` 같은 식으로 쓴다. 이 표현은 트레이트 바운트 문법의 syntatic sugar다. 단순한 상황에서 유용하다.
-   `+` 구문을 쓰면 다수의 트레이트 바운드를 줄 수 있다. `&(impl Summary + Display)` / `<T: Summary + Display>` 같은 식이다. 파라미터는 두 트레이트를 구현해야만 한다.
-   `where`를 쓰면 복잡한 상황에서 유용하다.

```rust
//트레이트 바운드를 줄 수 있다.
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
//impl을 이용하면 좀 더 간략하게 쓸 수 있다.
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

//impl을 쓰면 다소 번거롭다.
pub fn notify(item1: &impl Summary, item2: &impl Summary) {}
//트레이트 바운드를 쓰면 훨씬 간략하다.
pub fn notify<T: Summary>(item1: &T, item2: &T) {}

//+ 구문
pub fn notify(item: &(impl Summary + Display)) {}
pub fn notify<T: Summary + Display>(item: &T) {}

//where
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{}
```

#### 리턴 타입과 트레이트

-   리턴 타입에도 `impl Summary` 같은 식으로 쓸 수 있다. 이 경우 정확한 타입을 기입하지 않는다. 그럴 필요가 있다면 트레이트를 리턴할 이유가 없으니까.
-   그런데 `impl`은 단일 타입을 리턴할 때만 사용이 가능하다.

```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
```

```rust
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            // snip
        }
    } else {
        Tweet {
            // snip
        }
    }
}
// 컴파일 안 된다.
// 이는 `impl Trait`의 구현과 연관된 문제라고 한다. 다른 방법이 있다고 한다.
```

#### 트레이트 바운드를 이용한 조건부 구현

-   아래 예시들처럼 조건부 구현도 가능하다.

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

struct NoImpl {
    x: u8,
}

fn main() {
    //c는 cmp_display를 쓸 수 있다.
    let c = Pair { x: 1, y: 1 };
    //NoImple은 어느 트레이트도 구현하지 않으므로 cmp_display를 쓸 수 없다.
    let c2 = Pair { x: NoImpl{ x: 1}, y: NoImpl{ x: 1} };
}
```

-   임의의 타입에 조건부 구현을 하는 방법도 존재한다. `blanket implementation`이라고 하며 스탠다드 라이브러리에서 많이 쓴다. `ToString`이 대표적이라고.
-   이런 구현에 대한 상세는 트레이트 문서의 "Implementors" 섹션에서 찾을 수 있다고 한다.

```rust
impl<T: Display> ToString for T {
    // --snip--
}

let s = 3.to_string();
```

### 라이프타임(Lifetime)

-   레퍼런스가 유요한 스코프를 의미한다. 즉, 모든 레퍼런스는 라이프타임을 가진다.
-   대부분 타입처럼 알아서 추론되지만, 여러 가능성이 있어 불가능한 경우에는 타입처럼 명시해주어야한다.
-   라이프타임의 주목적은 dangling reference를 방지하는 것이다.

```rust
fn main() {
    //할당 안하고도 쓸 수 있다. 그런데 사용전에는 할당 해야한다. 이는 rust에 null이 없기 때문이다.
    let r;

    {
        let x = 5;
        r = &x;
           //^^ borrowed value does not live long enough
    }
    // - `x` dropped here while still borrowed

    println!("r: {}", r);
    // - borrow later used here
}
```

#### borrow checker

-   컴파일러는 borrow checker를 포함한다. 이를 통해 스코프를 비교하고 모든 `borrow`가 유효한지 확인한다. 아래 예시처럼 동작한다.

```rust
// 1. r은 'a의 스코프를, x는 'b의 스코프를 가진다.
// 2. r은 'a의 라이프타임을 가지지만 그보다 작은 'b 라이프타임(x)의 메모리를 참조한다.
// 3. 그렇기에 이 프로그램은 유효하지 않고, 컴파일 되지 않는다.
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
```

```rust
// 1. r은 'a의 스코프를, x는 'b의 스코프를 가진다.
// 2. r은 'a의 라이프타임을 가지고, 그보다 큰 'b 라이프타임(x)의 메모리를 참조한다.
// 3. 그렇기에 이 프로그램은 유효하고 컴파일 된다.
fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+
```

#### 라이프타임 명시

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
// 컴파일 안 된다.
// 왜냐하면 두 레퍼런스중 뭐가 리턴될 지 알 수 없기 때문이다.
// 때문에 라이프타임도 알 수 없다.
fn longest(x: &str, y: &str) -> &str {
    //        ----     ----     ^ expected named lifetime parameter
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

-   `'`를 이용해 라이프타임을 명시할 수 있다. 이름 앞에 붙이며, 보통 짧게 전부 소문자로 이름을 붙인다.
-   외부로부터 유래한 레퍼런스(파라미터)가 유발하는 라이프타임 관계를 컴파일러에게 정확히 알려주기 위해 사용한다.
-   정보를 주는 어노테이션이지 실제 레퍼런스의 수명을 길게하는 등, 라이프타임에 영향을 주는 동작은 할 수 없다.

```rust
&i32        // 레퍼런스
&'a i32     // 라이프타임을 명시한 레퍼런스
&'a mut i32 // 라이프타임을 명시한 가변 레퍼런스
```

-   위의 라이프타임 명시만으로는 별 의미가 없다. 아래처럼 함수의 맥락에서 의미가 있다.
-   제네릭처럼 명시한다.

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// 리턴과 파라미터의 라이프타임이 동일하다고 컴파일러에 힌트를 준다.
// 동작을 바꾸지 않는다. borrow checker는 이 라이프타임을 지키지 않는 값을 거절한다.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

```rust
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        //이렇게 정확한 레퍼런스가 넘어갈 때 라이프타임이 확정 되는데
        //이 경우, string2의 라이프타임이 짧으므로, 해당 함수 내에서 string1의 라이프타임이 string2와 같은 것으로 간주된다.
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

```rust
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        // strign2의 라이프타임이 가장 짧으므로 그에 맞춰진다.
        // 컴파일러가 인식하기에 result의 라이프타임도 이 스코프까지다.
        result = longest(string1.as_str(), string2.as_str());
    }
    //그러니 borrow checker는 이 코드가 무효하다고 판단한다.
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

```rust
//y는 x의 라이프타임과 관계를 전혀 맺지 않기 때문에 필요없다.
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

```rust
//이런 dangling reference 상황도 불가능하다.
//외부 요인인 파라미터들의 라이프타임과 리턴값의 라이프타임이 서로 아무 관계도 없기 때문이다.
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
```

#### struct와 라이프타임

-   struct가 레퍼런스 필드를 지닐 수도 있는데 이 경우, 라이프타임을 명시해야한다.

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    //part는 novel의 라이프타임을 가진다고 명시한다.
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

#### 라이프타임 생략 규칙(lifetime elision rule)

-   rust는 라이프타임에 한하여 간략한 코드 작성을위해 생략 규칙을 가진다. 이를 통해 명확한 상황에서 컴파일러가 알아서 라이프타임을 추론하게 해주고 작성할 코드 수를 줄여준다.
-   파라미터의 라이프타임은 `인풋 라이프타임`, 리턴 값의 라이프타임은 `아웃풋 라이프타임`이라고 한다.
-   현재 컴파일러는 다음 세 가지 규칙으로 라이프타임을 추론한다. 이 규칙을 만족하지 못하면 라이프타임을 명시해야 한다고 컴파일러가 알려준다.
    1. 컴파일러는 라이프타임 파라미터를 각 레퍼런스 파라미터에 할당한다. `fn foo<'a>(x: &'a i32);`, `n foo<'a, 'b>(x: &'a i32, y: &'b i32);` 같은 식이다.
    2. 만약 딱 하나의 인풋 라이프타임 파라미터가 있다면 라이프타임이 모든 아웃풋 라이프타임 파라미터에 적용 된다. `fn foo<'a>(x: &'a i32) -> &'a i32`.
    3. 다수의 인풋 라이프타임 파라미터가 존재하는데 그중 하나가 `&self`이거나, `&mut self`인 경우, 즉, 메서드인 경우, `self`의 라이프타임이 모든 아웃풋 라이프타임 파라미터에 적용된다.

```rust
//사실 아래의 코드는 과거 rust에서는 컴파일 되지 않았다.
//하지만 생략 규칙이 추가 되면서 이렇게 작성이 가능해졌다.
//앞으로도 많은 규칙이 추가되어 라이프타임을 기술하는 경우는 더 적어질 것이다.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

```rust
fn first_word(s: &str) -> &str {}
//1. 각 레퍼런스 파라미터가 자신의 인풋 라이프타임 파라미터를 갖게한다.
//fn first_word<'a>(s: &'a str) -> &str
//2. 딱 하나의 인풋 라이프타임 파라미터가 존재하므로, 아웃풋 라이프타임 파라미터도 이와 동일하다.
//fn first_word<'a>(s: &'a str) -> &'a str
//3. 모든 인풋과 아웃풋이 라이프타임을 알게 되었으므로 3번 규칙은 볼 필요 없다. 애초에 self도 없다. 이대로 borrow checker가 라이프타임을 체크한다.
```

```rust
fn longest(x: &str, y: &str) -> &str{}
//1. 각 레퍼런스 파라미터가 자신의 인풋 라이프타임 파라미터를 갖게한다.
// fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str
//2. 두 개의 인풋 라이프타임 파라미터가 존재한다. 아웃풋 라이프타임 파라미터를 추론할 수 없다.
//3. self가 없으므로 3번 규칙을 적용할 수 없다.
//모든 인풋/아웃풋 파라미터의 라이프타임을 추론할 수 없으므로 컴파일러는 해당 함수의 라이프타임을 추론할 수 없다.
```

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

//impl과 타입에는 명시해야한다.
//1. 레퍼런스 파라미터가 자신의 인풋 라이프타임 파라미터를 갖게한다. 그런데 애초에 레퍼런스가 인풋 파라미터 하나라 모든 라이프타임을 추론할 수 있다.
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    //1. 레퍼런스 파라미터가 자신의 인풋 라이프타임 파라미터를 갖게한다.
    //2. 두 개의 인풋 라이프타임 파라미터가 존재한다. 아웃풋 라이프타임 파라미터를 추론할 수 없다.
    //3. self가 존재한다. 아웃풋 라이프타임 파라미터는 self를 따라간다. 모든 파라미터의 라이프타임을 추론할 수 있다.
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

#### static 라이프타임

-   프로그램이 살아있는 동안 유효한 라이프타임을 의미한다.
-   대표적으로 문자열 리터럴들이 그에 속한다.
-   가끔 이 걸 쓰도록 권하는 경우가 있는데 그래도 정말 이 레퍼런스가 그러한지 잘 생각해본다.

```rust
let s: &'static str = "I have a static lifetime.";
```

### 제네릭, 트레이트 바운드, 라이프타임

```rust
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

### 테스트 코드 작성

-   테스트는 `#[test]` 어트리뷰트를 쓴 함수다.
-   테스트를 할 땐 `cargo test` 커맨드를 쓴다. 그러면 Rust가 알아서 테스트 러너 바이너리를 만들고 테스트를 진행한다.
-   새 라이브러리 프로젝트를 만들 떄(`cargo new [project name] --lib` 테스트 모듈이 알아서 만들어진다. 이 모듈은 테스트 템플릿 코드로 쓰인다.
-   검증을 위해 다양한 assert 매크로가 제공된다.
-   assert 메서드를 위해 자기가 작성한 struct와 enum에 `#[derive(PartialEq, Debug)]`을 다는게 좋다.
-   벤치마크도 가능하다는데 나이틀리 빌드의 rust에서만 되는 거 같다.
-   문서화 테스트도 한다.
-   `#[should_panic]`으로 패닉이 발생해야만 하는 테스트도 할 수 있다. `panic!` 자체에 메시지를 담는 것도 유용할 수 있다.
-   `Result<T,E>`는 assert 없이도 바로 테스트가 가능하다. 패닉을 발생시키고 싶으면 `assert!(value.is_err())`를 쓰자.
-   테스트 필터링도 할 수 있고, 무시도 할 수 있다.
-   `carge test --help` 테스트 시 쓸 수 있는 옵션들을 볼 수 있다.
-   테스트는 기본적으로 스레드를 사용해 병렬로 실행된다. `--test-threads=1` 플래그를 이용해 스레드 갯수를 조절할 수도 있다.
-   보통은 실패한 테스트의 stdout만 캐치되고 결과창에 보여진다. 성공한 것도 보고 싶으면 `--show-output` 플래그를 쓴다.
-   `cargo test [테스트 이름]`을 입력하면 하나의 테스트만 실행할 수 있다.
-   `cargo test add`를 입력하면 이름에 add가 포함된 테스트만 실행할 수 있다.
-   `#[ignore]`를 쓰면 해당 테스트를 무시한다. `--ignored` 플래그를 쓰면 이런 테스트만 실행한다. `--include-ignored`는 그냥 다 실행한다.
-   유닛 테스트시, 테스트 모듈의 이름은 보통 `tests`로 짓고 `#[cfg(test)]` 어트리뷰트를 붙인다. 이 어트리뷰트는 해당 모듈의 코드를 `cargo test`을 실행할 때만 실행하라고 알려주는 역할을 한다. 이를 통해 필요한 부분만 컴파일 할 수 있다. 테스트는 바이너리에 포함 되지 않는다. 유닛 테스트는 테스트할 코드와 같은 파일에 저장 되기에 필요한 매커니즘이다.
-   통합 테스트는 루트에 `tests` 디렉토리를 만들어 진행한다. `Cargo`가 인식하는 폴더다. 해당 폴더내 각 소스가 하나의 크레이트가 된다. 해당 디렉토리는 특별하기에 `cargo test`할 때만 실행되고 컴파일 된다. 그러므로 `#[cfg(test)]`가 통합 테스트에선 필요 없다.
-   통합 테스트시 공통 루틴을 정의한 소스에 대해서는 오래된 방식의 모듈을 쓴다(`test/common/mod.rs`). 그래야 테스트에 안 걸린다.
-   만일 프로젝트에 라이브러리 크레이트가 없으면(src/lib.rs) 통합테스트를 만들 수 없다.

```rust
//src/lib.rs
//'cargo new'로 생성된 것
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

```rust
#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`",
    result
    );
}
```

```rust
#[test]
#[should_panic(expected = "less than or equal to 100")]
fn greater_than_100() {
    Guess::new(200);
}
```

```rust
#[test]
fn it_works() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}
```

### 함수형 프로그래밍 스타일로 코드 작성

#### 클로저(Closure)

-   rust는 클로저라는 이름의 익명함수를 제공한다. 이 함수는 변수에 저장할 수도 있고, 다른 함수에서 파라미터로도 쓸 수 있다.
-   값을 스코프내에 캡쳐한다.
-   보통 타입을 쓰지 않는다. 이는 클로저가 노출되는 인터페이스가 아니기 때문이다.
-   클로저는 이처럼 이름도 없고, 외부로 노출 되지도 않는데다 어떤 상황이든 굉장히 좁은 컨텍스트에서 사용된다. 그렇기에 컴파일러가 그 시그니처를 추론하는게 어렵지 않다. 물론 거의 안 쓸 뿐이지 컴파일러가 요구하는 순간도 있다.
-   클로저는 세가지 방식으로 변수를 캡쳐한다.
    -   불변 레퍼런스(borrowing immutably): 문맥에 따라 컴파일러가 추론한다.
    -   가변 레퍼런스(borrowing mutably): 문맥에 따라 컴파일러가 추론한다.
    -   소유권 취득(take ownership): 문맥에 따라 그렇게 추론 되지 않더라도, `move` 키워드로 강제로 소유권을 취득할 수 있다. 새 스레드로 데이터를 넘길 때 사용하는 테크닉이다.
-   클로저의 바디는 사실상 \*캡쳐한 값을 나중에 쓸 때 어떻게 쓸지 정의한 코드\*\*이다. 클로저가 값을 캡쳐하고 다루는 방법은 클로저가 구현하는 트레이트에 영향을 미친다. 이 트레이트는 함수와 struct가 사용할 수 있는 클로저의 종류를 지정하는 방법이다. 클로저는 바디가 캡쳐한 값을 다루는 방법에 따라 알아서 `Fn` 트레이트를 하나에서 세개까지 구현한다.
    -   `FnOnce`: 한번 호출 될 수 있는 클로저에 적용된다. 모든 클로저가 한 번이상 호출 될 수 있기에 이 트레이트를 구현한다. 바디 밖으로 캡쳐한 값을 move하는 클로저는 이 것만 구현한다. 딱 한 번만 호출될 테니까.
    -   `FnMut`: 캡쳐된 값을 바디 밖으로 move하지 않는 클로저에 구현된다. 하지만 값을 변경할 수도 있다. 한 번 이상 호출될 수 있다.
    -   `Fn`: 캡쳐된 값을 바디 밖으로 move 하지도 않고,변경하지도 않는 클로저에 구현된다. 클로저가 아무것도 캡쳐하지 않는 것과 같다. 한 번이상 호출 될 수 있으며 캡쳐한 값을 바꾸지 않는다. 동시에 클로저를 여러번 호출할 때 중요하다.
-   함수도 `Fn` 트레이트 세 개를 전부 구현할 수도 있다. 가령, 캡쳐를 안 할거라면 원하는 `Fn` 트레이트를 구현한 함수를 쓸 수도 있다. 가령, `Option<Vec<T>>` 값에 대해 `unwrap_or_else(Vec::new)`를 호출했다면 `None`의 경우, 빈 배열이 리턴된다.

```rust
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // unwrap_or_else는 클로저를 파라미터로 받아들인다.
        // 이 클로저는 파라미터가 없기에 || ~의 형태이며, self를 캡쳐한다.
        // 보다시피 이렇게 굉장히 제한적인 컨텍스트에서 사용된다.
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
```

```rust
//이렇게 타입을 명시한다.
//이렇게 변수에 할당할 수도 있다.
let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
```

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 } // 함수
let add_one_v2 = |x: u32| -> u32 { x + 1 }; // 클로저
// 사용 문맥이 없으면 타입을 표시해야한다. 이는 `let v = Vec::new()`와 동일하다.
let add_one_v3 = |x|             { x + 1 }; // 클로저
let add_one_v4 = |x|               x + 1  ; // 클로저
```

```rust
let example_closure = |x| x;

let s = example_closure(String::from("hello"));
//이미 s에서 이 클로저는 String에 대한 것으로 추론이 끝났다.
let n = example_closure(5);
//error[E0308]: mismatched types
//expected struct `String`, found integer
```

```rust
fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    //println!에는 불변 레퍼런스 캡쳐면 충분하다.
    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    //캡쳐 후 나중에 쓴다.
    only_borrows();
    println!("After calling closure: {:?}", list);
}
```

```rust
fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    //push를 하려면 가변 레퍼런스 캡쳐
    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);
}
```

```rust
fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    //list의 소유권을 메인 스레드에서 새 스레드로 넘긴다.
    //어떤 스레드가 먼저 끝날지 알 수 없기 때문에 이런 동작을 해야한다.
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}
```

```rust
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        //클로저가 한번만 호출된다.
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];
    //FnMut를 구현한다. 이 이유는 정렬할 때 슬라이스내 각 아이템 횟수만큼 호출하기 때문이다.
    //FnMut는 변경할 '수도' 있음이라는 걸 기억해라.
    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
}
```

```rust

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut sort_operations = vec![];
    let value = String::from("by key called");

    //push로 인해 소유권이 옮겨진다. 이는 FnOnce를 구현한 것과 같다.
    //그런데 해당 함수는 소유권이 옮겨지므로 한 번만 호출할 수 있다.
    //그러므로 여러번 호출 될 수도 있는 이 함수에는 적용될 수 없다.
    list.sort_by_key(|r| {
        sort_operations.push(value);
        r.width
    });
    println!("{:#?}", list);
}
```

#### 반복자(Iterator)

-   기본적으로 `lazy`하다. 반복자를 실제 소모하는 코드를 만나기전까지는 아무것도 하지 않는다.
-   모든 반복자는 스탠다드 라이브러리의 `Iterator` 트레이트를 구현한다.
-   반복자 호출시 내부 상태가 변하기 때문에 `next`를 호출하려면 반드시 mut로 선언해야한다. `for` 루프는 내부적으로 반복자의 소유권을 가져가고 알아서 mut로 만들어 쓴다.
-   반복자의 메서드는 파라미터로 주로 클로저를 받아들인다.
-   `next`를 호출하는 메서드를 `consumng adaptor`라고 한다. 반복자에서 얻은 값을 실제로 사용하기 떄문이다. 대표적으로 `sum`이 있는데 이 메서드는 반복자의 오너쉽을 취득하고 반복적으로 `next`를 호출한다.
-   그와 반대로 `iterator adaptor`라는 메서드도 있다. 실제로 반복자를 쓰지 않고 대신 새로운 반복자를 만들어낸다. `map`이 대표젹이다. 다만 이런 메서드들은 실제 새로 생성한 반복자를 사용하기 전까지는 아무것도 하지 않는다. 이는 반복자의 lazy한 특성덕분이다.
-   반복자를 얻어올 땐, `iter`, `into_iter`, `iter_mut`등 다양한 반복자가 있다. 각각 불변 레퍼런스, 소유권 취득, 가변 레퍼런스를 의미한다.

```rust
let v1 = vec![1, 2, 3];
let v1_iter = v1.iter();
//이 것만으론 아무것도 하지 않는다.
```

```rust
let v1 = vec![1, 2, 3];
let v1_iter = v1.iter();

for val in v1_iter {
    //여기서 실체화가 된다.
    println!("Got: {}", val);
}
```

```rust
#![allow(unused)]
fn main() {
    pub trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;

        // methods with default implementations elided
    }
}
```

```rust
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    //반복자 호출시 내부의 상태가 변하기 때문에 mut로 선언해야한다.
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
```

```rust
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
    //다시 할 수 없다. sum에서 이미 반복자 소유권이 넘어갔기 때문이다.
    //let v1_iter = v1.iter(); 새로 다시 만든다.
}
```

```rust
let v1: Vec<i32> = vec![1, 2, 3];

//새 반복자를 소모하기전까지는 아무것도 안 한다.
v1.iter().map(|x| x + 1);
//warning: unused `Map` that must be used
//note: iterators are lazy and do nothing unless consumed
```

```rust
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    //filter가 소유권을 취득하므로 into_iter를 쓴다.
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}

```

#### 루프와 반복자 성능 비교

-   반복자는 rust의 `zero-cost abstraction`중 하나다. 즉, 비슷한 로우 레벨 코드와 거의 비슷하게 컴파일 되며 런타임 오버헤드가 없다.
-   성능 걱정하지말고 마구 써라.

> 통상적으로 C++의 구현은 제로 오버헤드의 원칙을 따른다.
> "쓰지 않는 것에 대가를 치르지 않는다. 더욱이, 쓰는 것보다 더 나은 코드를 사용할 수 없다."
> _Bjarne Stroustrup, Foundations of C++” (2012)_

```rust
let buffer: &mut [i32];
let coefficients: [i64; 12];
let qlp_shift: i16;

for i in 12..buffer.len() {
    //rust는 이런 코드를 루프로 만드는 짓을 하진 않을 것이다.
    //통상 루프 언롤링을 해 있는 그대로 로우레벨 코드로 컴파일 한다.
    //그러니까 모든 coefficients내 값이 레지스터에 들어가서 연산될 거고, 배열의 바운드 체크 코드 같은 건 들어가지 않는다.
    let prediction = coefficients.iter()
                                 .zip(&buffer[i - 12..i])
                                 .map(|(&c, &s)| c * s as i64)
                                 .sum::<i64>() >> qlp_shift;
    let delta = buffer[i];
    buffer[i] = prediction as i32 + delta;
}
```

### Cargo와 [Crates.io](https://crates.io)

-   [상세 Cargo 가이드(https://doc.rust-lang.org/cargo/)

#### 릴리즈 프로필로 빌드 커스터마이즈

-   cargo는 `dev` 프로필과 `release` 프로필 두 가지를 사용한다. 전자는 `cargo build`, 후자는 `cargo build --release` 커맨드에 의해 사용된다.
-   `Cargo.toml`에 `[profle.dev]`와 `[profile.release]`로 설정한다.

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

#### Crate.io에 크레이트 배포

##### 문서화

-   documentation comment은 `///`를 쓰며 마크다운을 지원하며, 그를 통해 HTML을 생성한다.
-   `cargo doc [--open]`으로 문서를 만들며, *target/doc*에 생성된 파일이 담긴다.
-   커뮤니티에서는 보통 `# Examples`, `# Panics`, `# Errors`, `# Saftey`를 많이 작성한다. 각각, 예시, 패닉이 발생하는 상황, `Result` 리턴 시 발생할 수 있는 에러, 이 함수가 왜 `unsafe`이고, 어떻게 다루어야하는지를 나타낸다.
-   `# Example` 코드블록의 경우, 놀랍게도 `cargo test` 실행시 해당 코드를 실행한다. 그러니 동작되는 코드만 넣어두는 게 좋다.

````rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
````

-   외에도 `//!`가 있는데 이는 보통 전체 크레이트나 모듈에 대한 전반적인 설명을 작성하기 위해 사용한다. 보통 *lib.ru*의 최상단에 작성한다.

```rust
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
// --snip--
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

##### pub use로 사용자 친화적인 퍼블릭 API 구조 생성

-   사용자 입장에서는 `my_crate::some_module::another_module::UsefulType;` 보다는 `use my_crate::UsefulType;`가 나을 것이다.
-   사용자 친화적인 모듈구조를 만들기 위해 기존 구조를 수정할 필요는 없다. `pub use`를 사용하면 원하는 구조로 모듈을 익스포트할 수 있다(re-export).

```rust
//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor; //이러면 Re-export로 문서에 나오는데다 art::PrimaryColor로 바로 임포트가 가능하다.
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    // --snip--
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    // --snip--
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Orange
    }
}
```

##### Crate.io에 배포

```bash
cargo publish
```

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"
```

#### 워크스페이스

-   같은 _Cargo.lock_ 파일과 아웃풋 디렉토리를 공유하는 패키지 모음을 말한다. VS의 솔루션과 프로젝트를 생각하면 될 거 같다.
-   새로운 `Cargo.toml`을 만든다. 이 파일은 워크스페이스를 정의한다.
-   `cargo run -p [패키지 이름]`으로 바이너리를 실행할 패키지를 지정할 수 있다.
-   `cargo test`는 모든 패키지를 테스트한다. 마찬가지로 `-p`를 쓰면 패키지를 지정할 수 있다.

```toml
# Cargo.toml
[workspace]

members = [
    "adder",
    "add_one",
]
```

```
├── Cargo.lock # 단 하나만 존재한다.
├── Cargo.toml
├── add_one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target # 단 하나만 존재한다.
```

```toml
# adder/Cargo.toml
[dependencies]
add_one = { path = "../add_one" }
```

#### `cargo install`

-   바이너리를 로컬에 인스톨할 수 있다.
-   보통 *$HOME/.cargo/bin*에 설치된다.

#### Cargo 확장

-   원한다면 커맨드 확장도 가능하다.
-   `cargo-something`이란 파일이 \*$PATH&에 있으면 `cargo something`으로 실핼할 수 있는 식이다. `cargo --list`로 확인 가능.

### 스마트 포인터

-   보통 struct로 구현되고, `Deref`와 `Drop` 트레이트를 구현.
-   `Deref`는 레퍼런스처럼 동작하게 해주는 트레이트이다.
-   `Drop`은 스코프를 벗어난 데이터의 공간을 할당 해제해준다.
-   대표적으로 `Box<T>`, `Rc<T>`, 그리고 `RefCell<T>`가 있다.
    -   소유자의 수: `Box<T>`와 `RefCell<T>`는 소유자가 한명, `Rc<T>`는 소유자가 여러명
    -   borrow 규칙 체크 시기: `Box<T>`와 `Rc<T>`는 컴파일 타임(이 중 `Rc<T>`는 불변 레퍼런스만 사용). `RefCell<T>`는 런타임.

#### Box<T>

-   데이터를 힙에 할당할 수 있게 해주는 스마트 포인터.
-   데이터를 힙에 저장하는 것 외에는 퍼포먼스 오버헤드가 없다.
-   자동으로 할당 해제된다. 스택의 포인터도 물론.
-   다음 상황에서 주로 쓴다.
    -   컴파일 타임에 크기를 알 수 없는 타입을 그 크기를 모르는 컨텍스트에서 써야할 때(포인터의 크기는 알 수 있다.)
    -   복사 시간이 오래 걸리는 크기가 큰 데이터의 소유권을 복사 없이 옮기고 싶을 때
    -   값의 소유권을 갖길 원하면서 그 타입이 특정 트레이트를 구현했는지에만 관심있을 때(트레이트 오브젝트)

```rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```

##### recursive type

-   타입 내에 값이 그 자신의 타입을 갖는 타입.
-   컴파일 타임에 정확한 데이터 크기를 알 수 없다.
-   대표적으로 **cons list** 구조가 있다. 이 구조는 `Lisp`로부터 유래했으며 함수형 프로그래밍 스타일에서 많이 쓰인다. 중첩된 페어 구조를 지니며, Lisp의 링크드 리스트라고 할 수 있다. 두 개의 값을 지니는데 하나는 현재 아이템의 값이고, 다른 하나는 다음 아이템이다.

```lisp
(1, (2, (3, Nil)))
```

```rust
enum List {
// ^^^^^^^^^ recursive type has infinite size
    Cons(i32, List),
             //---- recursive without indirection
    Nil,
}
// help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `List` representable
// indirection은 값을 직접 저장하지말고 포인터로 간접적으로 저장하란 의미다.

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
```

```rust
enum Message {
    //이중 가장 큰 사이즈의 variant가 할당될 크기다.
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

```rust
enum List {
    //모든 크기를 컴파일러가 알 수 있다.
    Cons(i32, Box<List>),
    Nil,
}
```

#### Deref 트레이트

-   해당 트레이트는 타입이 `*` 연산자(dereference)의 동작을 커스터마이징할 수 있게 해준다.
-   그렇기에 타입에 이 트레이트를 구현함으로써 스마트 포인터가 레퍼런스처럼 동작하게끔 만든다.
-   `DerefMut` 트레이트도 존재한다. 이는 가변 레퍼런스를 위해 존재한다.

```rust
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    //assert_eq!(5, y);
    //^^^^^^^^^^^^^^^^ no implementation for `{integer} == &{integer}`
}
```

```rust
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    // Box도 derefernece가 된다.
    assert_eq!(5, *y);
}
```

```rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    //error[E0614]: type `MyBox<{integer}>` cannot be dereferenced
}
```

```rust
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T; // associated type이라고 한다.

    //dereference 동작 구현
    // *y -> *(y.deref())로 알아서 바꿔준다.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

#### Deref Coercion

-   `Deref` 트레이트를 구현한 타입을 레퍼런스로 바꿔주는 동작을 말한다.
-   `&String` -> `&str`로 알아서 바꿔주는 게 대표적인 예시다.
-   요구하는 타입과 일치할 때까지 `Deref::deref()`가 호출된다.
-   다음과 같은 경우에 해당 동작이 벌어진다.
    -   `T: Deref<Target=U>` 일 때, `&T` -> `&U`로(T가 U에 대해서 Deref를 구현했을 때)
    -   `T: DerefMut<Target=U>` 일 때, `&mut T` -> `&mut U`로
    -   `T: Deref<Target=U>` 일 때, `&mut T` -> `&U`로(이 역은 성립하지 않는다. 가변 레퍼런스가 존재할 때 불변 레퍼런스는 존재할 수 없기 때문이다.)

```rust
fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    //*(m.deref())
    hello(&m);
}
```

```rust
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    //Deref가 구현이 되어있지 않다면...
    hello(&(*m)[..]);
}
```

#### Drop 트레이트

-   값이 스코프를 벗어낫을 때 동작을 커스터마이징할 수 있게 해준다.
-   가변 레퍼런스 `self`를 파라미터로 받는 `drop()`을 구현한다. 이 메서드는 값이 스코프를 벗어나면 알아서 호출된다.
-   모든 값은 역순으로 해제된다.
-   prelude에 포함되어있다.

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}

```

##### 인위적으로 drop 호출

-   다음처럼 인위적으로 `drop()`을 호출하는 건 불가능하다.
-   이는 rust가 어차피 알아서 `drop()`을 호출하기 때문이다. 명시적으로 해제하면 double free 문제가 생긴다.
-   `std::mem::drop`를 이용하면 rust가 알아서 `drop()`을 호출하는 타이밍을 인위적으로 앞당길 수 있다. 참고로 이 함수는 prelude다.

```rust
fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    c.drop();
  //--^^^^--
  //explicit destructor calls not allowed
    println!("CustomSmartPointer dropped before the end of main.");
}
```

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
```

#### Rc<T>

-   레퍼런스 카운팅용 스마트 포인터.
-   해당 타입은 싱글 스레드 시나리오에서만 사용한다.
-   하나의 값이 여러 소유자를 가지고 있을 때 사용한다. 대표적으로 그래프 구조가 있다. 여러 에지가 하나의 버텍스를 가리킬 수 있기 떄문에 버텍스를 함부로 해제할 수 없다.
-   불변 레퍼런스이므로 데이터 소유자가 여러명이어도 상관없다(readonly).
-   스트롱 레퍼런스의 갯수로 할당 해제 여부를 검토한다.
-   `clone()`으로 레퍼런스 카운트를 올린다. 이 때 올라가는 카운트는 스트롱 레퍼런스로 `string_count()`로 그 갯수를 확인할 수 있다. 스트롱 레퍼런스는 값의 소유권을 공유한다.
-   위크 레퍼런스로도 만들 수 있는데 `downgrade()`를 호출하여 가능하다. `Weak<T>` 스마트 포인터를 리턴한다. 위크 레퍼런스가 남아있어도 할당 해제 되는데는 영향을 끼치지 않는다. 그래서 값을 얻어오는 `upgrade()` 메서드는 `Option<T>`를 리턴해 값이 유효한지 체크할 수 있게 해준다.

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
                            //- value moved here
    let c = Cons(4, Box::new(a));
                            //^ value used here after move
}
```

```rust
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // a.clone()이 아니라, Rc::clone()을 써야함을 주의.
    // 이래야 딥카피가 안 되기 때문이다.
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
```

```rust
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
/*
count after creating a = 1
count after creating b = 2
count after creating c = 3
count after c goes out of scope = 2
*/
```

#### RefCell<T>

-   borrowing 규칙을 런타임에 체크한다.
-   즉, 컴파일은 되어도 프로그램이 패닉에 빠지거나 꺼질 수도 있다.
-   어쨌든, borrowing 규칙은 지켜진다.
-   런타임에 체크를 하기 때문에 그 값에 불변 레퍼런스가 있더라도 값을 수정하는 코드를 작성할 수 있다.
-   이렇게 불변 레퍼런스가 있어도 가변 레퍼런스를 사용해 값을 바꿀 수 있게 하는 걸 `Interior mutability` 패턴이라고 한다.
-   싱글 스레드 시나리오에서만 쓴다.
-   코드가 borrowing 규칙을 따르는지 검증하고 싶으나 컴파일러가 모종의 이유로 해당 규칙을 검증할 수 없을 때 유용하다.
-   가령, 값이 그 외부 환경에서는 변경될 수 없지만, 그 메서드 안에서는 변경될 수 있게 하고 싶을 때 유용하다.
-   런타임에 borrow를 추적하므로 퍼포먼스상 불이익이 조금 있다.
-   `borrow()`와 `borrow_mut()` 메서드로 스마트 포인터를 리턴한다. 전자는 `Ref<T>`, 후자는 `RefMut<T>`를 리턴한다. 둘다 `Deref`를 구현한다. `RefCell<T>`는 이들의 갯수를 각각 추적한다. 이를 통해 런타임에 borrowing 규칙(_다수의 불변 레퍼런스가 아니면 단 하나의 불변 레퍼런스가 존재할 수 있다_)을 추적한다.

```rust
impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        let mut one_borrow = self.sent_messages.borrow_mut();
        let mut two_borrow = self.sent_messages.borrow_mut();
        // 두개의 가변 레퍼런스는 불가능하다!
        // thread 'main' panicked at 'already borrowed: BorrowMutError', src/lib.rs:60:53
        one_borrow.push(String::from(message));
        two_borrow.push(String::from(message));
    }
}
```

```rust
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;
    //이렇게 다수의 값을 한번에 바꾸는 응용도 가능하다.
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
```

> ### 주의!
>
> `*value.brrow_mut()`의 경우, `*`는 `value`가 아니라, `value.borrow_mut()`에 적용된다.

#### 레퍼런스 사이클과 메모리 누수

-   대부분의 상황에서 메모리는 누수 없이 잘 관리되겠지만, 아주 드물게 그렇지 못한 일이 벌어질 수 있다.
-   스마트 포인터를 사용해 레퍼런스 사이클이 벌어질 때 메모리 누수가 생길 수 있다.
-   보기 힘든 광경이지만, 발생하면 재앙이다. 예시처럼 `RefCell<T>`과 `Rc<T>`를 동시에 사용할 때 벌어질 확률이 큰데 이런 코드를 작성할 때는 레퍼런스 사이클이 발생하지 않도록 조심해야 한다.
-   `Weak<T>`를 활용하면 이런 경우를 피하는데 도움이 될 수 있다.

```rust
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        //a와 b 사이에 레퍼런스 사이클이 형성 된다.
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // 그러므로 출력을 하면 결국, 스택 오버플로우가 발생한다.
    // println!("a next item = {:?}", a.tail());
}
```

```rust
//https://jacksonshi.vercel.app/blog/rust/rc-weak-rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    next: Option<Rc<RefCell<Node>>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping");
    }
}

fn main() {
    let a = Rc::new(RefCell::new(Node {next: None}));
    println!("a count: {:?}",  Rc::strong_count(&a)); //1
    let b = Rc::new(RefCell::new(Node {next: Some(Rc::clone(&a))}));
    println!("a count: {:?}",  Rc::strong_count(&a)); //2
    println!("b count: {:?}",  Rc::strong_count(&b)); //1
    let c = Rc::new(RefCell::new(Node {next: Some(Rc::clone(&b))}));

    (*a).borrow_mut().next = Some(Rc::clone(&c));
    println!("a count: {:?}",  Rc::strong_count(&a)); //2
    println!("b count: {:?}",  Rc::strong_count(&b)); //2
    println!("c count: {:?}",  Rc::strong_count(&c)); //2
    // c -> b -> a -> c -> .... 의 사이클이 만들어진다.
    // 중요한 건 레퍼런스가 마지막에 도달하여 결국, c,b,a 순으로 해제가 되어도, Rc<RefCell<Node>>가 드랍 되지 않는다는 것이다.
    // 이미 각각의 카운트가 2인데, 아직 스트롱 레퍼런스가 1씩 남아있기 떄문이다.

    // 스택 오버플로우
    // println!("a {:?}",  &a);
}
```

```rust
//https://jacksonshi.vercel.app/blog/rust/rc-weak-rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    next: Option<Rc<RefCell<Node>>>,
    head: Option<Weak<RefCell<Node>>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping");
    }
}

fn main() {
    let a = Rc::new(RefCell::new(Node {next: None, head: None}));
    println!("a strong count: {:?}, weak count: {:?}", Rc::strong_count(&a), Rc::weak_count(&a));
    let b = Rc::new(RefCell::new(Node {next: Some(Rc::clone(&a)), head: None}));
    println!("a strong count: {:?}, weak count: {:?}", Rc::strong_count(&a), Rc::weak_count(&a));
    println!("b strong count: {:?}, weak count: {:?}", Rc::strong_count(&b), Rc::weak_count(&b));
    let c = Rc::new(RefCell::new(Node {next: Some(Rc::clone(&b)), head: None}));

    // Creates a reference cycle
    (*a).borrow_mut().head = Some(Rc::downgrade(&c));
    println!("a strong count: {:?}, weak count: {:?}", Rc::strong_count(&a), Rc::weak_count(&a));
    println!("b strong count: {:?}, weak count: {:?}", Rc::strong_count(&b), Rc::weak_count(&b));
    println!("c strong count: {:?}, weak count: {:?}", Rc::strong_count(&c), Rc::weak_count(&c));
    //a -> c에서 위크 레퍼런스가 형성된다. 즉, c가 해제 되는 순간 c의 스트롱 레퍼런스가 0이 될 것이고, 연쇄적으로 나머지 레퍼런스들이 해제될 것이다.

    println!("a {:?}",  &a);
}
```

### 동시성 프로그래밍

#### thread::spawn

-   `thread::spawn()`을 통해 스레드를 만들 수 있다. 해당 메서드는 실행할 클로저를 파라미터로 받고 `JoinHandle`을 리턴한다. 이를 통해 스레드 조인을 할 수 있다.

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
```

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        //println!은 소유권을 취한다.
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```

#### 메시지 패싱

-   채널을 사용해 스레드간 메시지로 데이터를 공유한다.
-   값에 대해 단일 소유권을 유지할 수 있다.
-   채널은 transimitter와 recevier의 개념을 가지고 있다. 말그대로 송신자, 수신자이고, 한쪽이라도 닫히면 채널은 드랍된다.
-   `mpsc::channel()`로 만들 수 있다. `mpsc`는 모듈이름인데 *multiple producer, single consumer*를 의미한다.

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel(); // 송신자, 수신자

    //송신자의 소유권을 옮겨줄 필요가 있다.
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        //send는 소유권을 요구한다.
        println!("val is {}", val);
                            //^^^ value borrowed here after move
    });

    //recv는 블로킹 함수, try_recv는 논블로킹 함수다.
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
```

```rust
    // --snip--

    let (tx, rx) = mpsc::channel();
    //이런 방식으로 다수의 프로듀서를 만들 수 있다.
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    // --snip--
```

#### 공유 메모리

-   일반적으로 스레드가 프로세스 내에서 값을 공유하는 방식. rust의 관점에선 값에 대해 **다수의 소유자를 허가**한다고 보면 된다.
-   `Mutex<T>`와 `Arc<T>`가 대표적인데 각각 `RefCall<T>`과 `Rc<T>`의 멀티 스레드 버전이다.
-   `Mutex<T>`는 interior mutability를 제공한다.
-   `Arc<T>`는 *atomically refernce counted type*을 의미한다고 한다. `Rc<T>`의 카운팅 로직이 스레드 세이프하지 않기 때문에 멀티 스레드에서는 쓸 수 없다. 참고로 rust는 lock-free 구현을 위한 `std::sync::atomic` 타입도 가지고 있다.
-   rust는 퍼포먼스를 위해 모든 타입을 굳이 atomic하게 구현하지 않는다.

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        //unwarp()을 통해 이미 락을 취득한 스레드가 패닉에 빠졌는지를 확인할 수 있다. 이 스레드도 패닉에 빠진다.
        //lock()은 스마트포인터 `MutextGuard`를 리턴한다.
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}
```

```rust
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        //`Rc<Mutex<i32>>` cannot be sent between threads safely
        //the trait `Send` is not implemented for `Rc<Mutex<i32>>`
        //Rc는 레퍼런스 카운터를 스레드 세이프하게 관리할 수 없다.
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

#### 동시성 기능 확장

-   rust 언어 자체로는 딱히 동시성 제어 기능을 제공하지 않는다. 지금껏 살펴본 건 스탠다드 라이브러리의 일부였을 뿐이다.
-   즉, 독자적인 동시성 기능을 구현할 수 있다는 의미가 되는데 `std:marker` 내의 `Sync`와 `Send` 트레이트로 가능하다.

##### `Send`

-   해당 트레이트를 구현한 타입 값의 소유권이 스레드간 이동이 가능한지를 나타낸다.
-   대부분의 rust 타입이 `Send`를 구현한다. `Rc<T>`는 이를 구현하지 않는 드문 예외다.
-   전부 `Send`로 이루어진 타입은 알아서 `Send`로 마킹된다.

##### `Sync`

-   다수의 스레드가 접근해도 안전함을 나타내는 트레이트다.
-   즉, 타입 `T`가 `Sync`를 구현하고, `&T`가 `Send`를 구현한다면 레퍼런스가 안전하게 스레드간 이동이 된다는 의미다.
-   전부 `Sync`로 이루어진 타입은 알아서 `Sync`로 마킹된다.
-   `Rc<T>`와 `RefCell<T>`는 `Sync`를 구현하지 않는다.

##### 그래서?

-   `Send`나 `Sync`로 이루어진 타입은 알아서 그걸 따라가므로 웬만해선 이런 트레이트를 직접 구현할 필요가 없다.
-   애초에 마킹 트레이트라 메서드도 없다.
-   알아서 조심하라는 걸까?

### OOP 기능

-   OOP 관점에서 rust를 살펴본다.

#### 트레이트 오브젝트

-   rust는 상속을 지원하지 않는다. 하지만 트레이트를 통해 OOP의 다형성을 구현할 수 있다.
-   트레이트 오브젝트는 **특정 트레이트를 구현한 타입의 인스턴스**, 또는 **런타임에 타입 내 트레이트 메서드의 룩업 테이블**을 의미한다. 전자는 너무 당연하니 여기서는 후자에 대해 알아본다.
-   & 레퍼런스나, `Box<T>` 같은 스마트포인터와 같은 **포인터**를 지정한 뒤, `dyn` 키워드를 이용해 트레이트 오브젝트를 만들 수 있다.
-   제네릭 타입 파라미터는 단 하나의 concrete 타입에 대해서만 사용할 수 있지만, 트레이트 오브젝트는 트레이트 정의를 만족하는 다수의 concrete 타입에 대해 사용할 수 있다. 일종의 덕 타이핑처럼 동작한다. 이 구체적인 타입은 런타임에 결정 되는 것으로 보인다.

```rust
pub trait Draw {
    fn draw(&self);
}

//Draw 트레이트를 구현한 T1,T2가 존재한다면,
pub struct Screen<T: Draw> {
    //components는 T1의 벡터이거나, T2의 벡터에 불과하다.
    pub components: Vec<T>,
}
impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

```rust
pub trait Draw {
    fn draw(&self);
}

//Draw 트레이트를 구현한 T1,T2가 존재한다면,
pub struct Screen {
    //벡터는 T1도, T2도 담을 수 있다.
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

```rust
//mod gui
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

//main
use gui::{Button, Screen};

fn main() {
    //components는 Draw 트레이트를 구현한 여러 타입을 담을 수 있는 벡터가 되었다.
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
```

> `static dispatch` vs `dynamic dispatch`
>
> 제네릭에 트레이트 바운드를 지정하면 컴파일 타임에 그에 해당 하는 코드를 만드는 `monomorphization` 과정이 벌어진다. 이런 과정으로 벌어지는 결과를 `static dispatch`라고 한다. 이 경우, 컴파일러가 컴파일 타임에 어떤 메서드를 호출할지 알고 있는 셈이다.
>
> 이와 반대 되는 개념이 `dynamic dispatch`인데 컴파일러가 컴파일 타임에 무엇을 호출하지 모르는 것이다. 즉, 런타임에 무엇을 실행할지 결정하는 것이다. 트레이트 오브젝트가 이런 동작을 유발한다. 어떤 메서드를 호출할지 런타임에 결정 하기 위해, 트레이트 오브젝트를 정의시 포인터를 사용하는 것이다. 하지만 당연히 런타임 성능에 불이익이 존재한다.

#### State 패턴 구현으로 알아보는 rust 활용

```rust
//정석적인 OOP 패턴으로 구현한 State 패턴
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

struct PendingReview {}

struct Published {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    //Published일 때만 content를 출력하고 싶다.
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
/**
 * 트레이트 오브젝트를 이용해 이런 구현을 할 수 있지만,
 * 실제 사용시 주의를 안 하면 content가 노출 되어버리는 등,
 * 생각보다 문제가 터질 확률이 높은 코드임에는 틀림 없다.
*/
```

```rust
// rust의 소유권을 이용해서 상태를 코드에 녹여내기
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    //다른 struct에는 없다.
    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    // --snip--
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    //상태 전환시 소유권을 넘겨 아예 이전 상태를 무효화 시킨다.
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    //상태 전환시 소유권을 넘겨 아예 이전 상태를 무효화 시킨다.
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
/**
 * 이렇게 소유권을 이용해 상태 전환 자체를 타입으로 녹여낼 수도 있다.
 * OOP 방식이 유일한 답은 아니다.
*/
```

```rust
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
```

### 패턴과 매칭

#### 패턴이 사용되는 곳

##### `match` arm

-   match는 `exhaustive`하게 값을 검증한다.

```rust
/*
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
*/
match x {
    None => None,
    Some(i) => Some(i + 1),
}
```

##### 조건부 `if let` 표현식

-   한 가지 패턴을 매칭할 때 유용하다.
-   `else` 같은 것과도 쓸 수 있어 `match` 보다 조금 더 유연하게 쓸 수 있다.
-   다만 `exhaustive`하게 값을 검증하지 않는다.

```rust
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}
```

##### `while let` 조건부 루프

-   `if let`과 유사하다.

```rust
let mut stack = Vec::new();

stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

##### `for` 루프

-   `for x in y`에서 `x`가 패턴이 된다.

```rust
let v = vec!['a', 'b', 'c'];

//(index, value)로 분해하는 셈
for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
}
```

##### `let` 구문

-   `let x = 5;`에서 x도 패턴이다.

```rust
//let PATTERN = EXPRESSION;
let (x, y, z) = (1, 2, 3);
```

##### 함수 파라미터

-   함수 이름이 패턴이다.

```rust
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}
```

#### Refutability

-   매칭에 항상 성공하는 패턴을 `irrefutable`하다고 하고, 매칭에 실패할 수 있는 패턴을 `refutable`하다고 한다. `let x = 5`의 `x`가 전자에 해당한다. 반면 `if let Some(x) = a_value`의 `Some(x)`는 후자에 해당한다.

```rust
let Some(x) = some_option_value;
// ^^^^^^^ pattern `None` not covered
//note: `let` bindings require an "irrefutable pattern", like a `struct` or an `enum` with only one variant
```

```rust
if let x = 5 {
// ^^^^^^^^^
//= note: `#[warn(irrefutable_let_patterns)]` on by default
//= note: this pattern will always match, so the `if let` is useless
    println!("{}", x);
};
```

#### 패턴 목록

##### 리터럴

```rust
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

##### 변수

```rust
    let x = Some(5);
    let y = 10;
    //match가 새로운 스코프를 정의한다.
    //그래서 Some(y) = Some(5)가 매칭이 되어버린다.
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
    //Matched, y = 5
    //at the end: x = Some(5), y = 10
```

##### `|`, multiple pattern

```rust
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
```

##### 레인지(`..=`)

```rust
let x = 5;

match x {
    1..=5 => println!("one through five"),
    _ => println!("something else"),
}

let x = 'c';

match x {
    'a'..='j' => println!("early ASCII letter"),
    'k'..='z' => println!("late ASCII letter"),
    _ => println!("something else"),
}
```

##### destructuring

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    //short-hand
    //let Point { x, y } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}
```

```rust
fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}
```

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {x} and in the y direction {y}"
            );
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => println!(
            "Change the color to red {r}, green {g}, and blue {b}",
        ),
    }
}
```

```rust
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        //이렇게 복합적인 패턴도 간단하게!
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change color to hue {h}, saturation {s}, value {v}"
        ),
        _ => (),
    }
}
```

```rust
let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
```

##### 값을 무시하는 패턴

-   `_`, 다른 패턴 안에 `_`, `_`로 시작하는 이름, 값의 나머지 부분을 무시하는 `..`.

```rust
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn main() {
    foo(3, 4);
}
```

```rust
let mut setting_value = Some(5);
let new_setting_value = Some(10);

match (setting_value, new_setting_value) {
    //Some인지만 확인
    (Some(_), Some(_)) => {
        println!("Can't overwrite an existing customized value");
    }
    _ => {
        setting_value = new_setting_value;
    }
}

println!("setting is {:?}", setting_value);
```

```rust
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, _, third, _, fifth) => {
        println!("Some numbers: {first}, {third}, {fifth}")
    }
}
```

````rust
fn main() {
    //x는 이제 쓰지 않는다. 경고가 사라짐.
    let _x = 5;
    let y = 10;
}

```rust
//컴파일 안 됨, s의 소유권이 _s로 이동!
let s = Some(String::from("Hello!"));

if let Some(_s) = s {
    println!("found a string");
}

println!("{:?}", s);

//컴파일 됨
let s = Some(String::from("Hello!"));

if let Some(_) = s {
    println!("found a string");
}

println!("{:?}", s);
````

```rust
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
    //x말고는 무시
    // y: _, z: _로 안 써도 됨.
    Point { x, .. } => println!("x is {}", x),
}
```

```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
}
```

```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        //어디까지 매칭해야하는지 애매함
        (.., second, ..) => {
       //--          ^^ can only be used once per tuple pattern
            println!("Some numbers: {}", second)
        },
    }
    //error: could not compile `patterns` due to previous error
}
```

##### 매치 가드

-   `match` 암에 붙는 추가적인 조건문.
-   조금 더 정밀하고 안전하게 매칭할 수 있는 대신 `exhaustiveness`는 사라짐.

```rust
let num = Some(4);

match num {
    Some(x) if x % 2 == 0 => println!("The number {} is even", x),
    Some(x) => println!("The number {} is odd", x),
    None => (),
}
```

```rust
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // 소유권이 이동하던 문제 해결
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
}
```

##### `@` 바인딩

-   `@` 연산자는 패턴에 매칭된 값을 해당 스코프에서 캡처할 수 있게 해준다.

```rust
enum Message {
    Hello { id: i32 },
}

let msg = Message::Hello { id: 5 };

match msg {
    Message::Hello {
        id: id_variable @ 3..=7,
    } => println!("Found an id in range: {}", id_variable),
    Message::Hello { id: 10..=12 } => {
        println!("Found an id in another range")
    }
    Message::Hello { id } => println!("Found some other id: {}", id),
}
```

### 고급 기능

#### unsafe

-   `unsafe`한 동작이 존재하는데는 두가지 이유가 있다. 하나는 당연히 프로그램을 실행하는 하드웨어와 OS가 그렇지 못하기 때문이고, 나머지 하나는 rust의 정적 분석기와 컴파일러는 굉장히 보수적이기 떄문이다. 그래서 동작할 수 있는 코드임에도 컴파일을 거절하는 경우가 있다.
-   `unsafe` 키워드를 통해 rust 컴파일러에게 해당 코드 블록이 안전하지 않음을 알린다.
-   안전하지 않은 코드는 다음 다섯가지 동작을 할 수 있다(*unsafe superpower*라고 부른다).
    -   포인터(raw) 할당 해제
    -   안전하지 않은 함수나 메서드 호출
    -   가변 static 변수 접근 및 수정
    -   unsafe 트레이트 구현
    -   `union` 필드에 접근.
-   `unsafe`는 버로우 체커를 비활성화하거나 rust의 세이프티 체크를 비활성화하지 않는다. 단지 위처럼 평소에는 할 수 없는 다섯가지 일을 할 수 있게 해줄 뿐이고, 컴파일러가 체크하지 않을 뿐이다. 여전히 rust의 세이프티 체크가 적용된다.
-   메모리 버그가 여기서 발생할 것이므로 가급적 이 부분을 작게 유지하는 게 중요하다.

##### 포인터(raw) 할당 해제

-   그냥 포인터를 말하는 거 같다.
-   `*const T`와 `*mut T`가 존재한다. 전자는 가리키는 값을 변경할 수 없는 걸 의미한다.
-   레퍼런스와 스마트포인터와 다르게 다음 동작이 가능하다.
    -   가변이든, 불변이든 원하는 갯수의 포인터를 원하는 만큼 선언할 수 있다.
    -   유효하지 않은 메모리 공간을 가리킬 수 있다.
    -   null이 될 수 있다.
    -   자동으로 할당 해제가 안 된다.
-   그럼에도 이런 위험한 짓을 하는 건 로우레벨 연산의 인터페이스가 그렇거나 _더 나은_ 퍼포먼스를 위해서이다.

```rust
fn main() {
    let mut num = 5;
    // 선언 자체는 할 수 있지만 unsafe 블록 없이는 deref가 불가능하다.
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

//error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
    //unsafe{
    println!("r1 is: {}", *r1);
                        //^^^ dereference of raw pointer
    println!("r2 is: {}", *r2);
                        //^^^ dereference of raw pointer
    //}
}
```

```rust
//실제 deref하기전엔 이런 것까지 된다.
let address = 0x012345usize;
let r = address as *const i32;
```

##### 안전하지 않은 함수나 메서드 호출

```rust
fn main() {
    //함수가 unsafe이므로, 굳이 바디 안에 unsafe를 쓰지 않아도 된다.
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
}
```

```rust
//T에 대한 메서드가 표준 라이브러리에 존재하는데 잘 생각해보면 unsafe한 동작인데 잘 동작한다.
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();

    assert!(mid <= len);

    //사실 슬라이스간 겹치는 부분이 없으므로 문제 될 것 없는 코드다.
    //하지만 rust가 판단할 때 가변 레퍼런스를 두 개만드는 동작이므로 버로우 규칙에 어긋난다.
    (&mut values[..mid], &mut values[mid..])
    //first mutable borrow occurs here
    //second mutable borrow occurs here
    //returning this value requires that `*values` is borrowed for `'1`
}
```

```rust
use std::slice;
//바깥에 unsafe를 안 씀으로써 함수 시그니처에는 unsafe가 없다.
//다만 이 같은 상황처럼 스스로가 보장할 수 있는 상황에서만 이래야 한다.
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    let mut vector = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut vector, 3);
}
```

```rust
fn main() {
    use std::slice;

    let address = 0x01234usize;
    let r = address as *mut i32;
    //이렇게 안전하지 못한 코드를 작성하지 않도록 주의
    let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
}
```

-   FFI시 unsafe가 필요할 확률이 다분하다. `extern`을 이용해 FFI를 하고, ABI 목록을 작성한다.

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

> 네임 맹글링 방지
>
> `#[no_mangle]` 어노테이션을 쓴다.
>
> ```rust
> //이렇게 단일 함수만 포함할 수 있다. 이 땐 unsafe가 없어도 된다.
>
> #[no_mangle]
> pub extern "C" fn call_from_c() {
>     println!("Just called a Rust function from C!");
> }
> ```

##### 가변 static 변수 접근 및 수정

-   일반적으로 rust는 전역변수를 지원하지 않는다. 소유권의 개념 떄문에 멀티 스레드 환경에서 데이터 레이스가 발생한다.
-   그런데 static 변수를 지원하긴 한다. 기본적으로 불변이며 참조하는 메모리 위치가 변하지 않는다.
-   하지만 `static mut`로 가변 선언을 하면 `unsafe`를 써야만 해당 변수를 사용할 수 있다.

```rust
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
```

##### unsafe 트레이트 구현

-   포인터를 다뤄야하는 `Send`와 `Sync` 트레이트가 대표적인 예시라는데?
-   메서드 중 하나라도 컴파일러가 unsafe하다고 하면 해야함.

```rust
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {}
```

##### union 필드에 접근

-   C의 union이요.

```rust
#[repr(C)]
union MyUnion {
    f1: u32,
    f2: f32,
}
```

#### 트레이트

#### 타입

#### 함수

#### 매크로

### 기타
