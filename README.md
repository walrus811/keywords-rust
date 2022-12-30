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
$ cargo new [프로젝트 이름]
$ cargo build [--release]
$ cargo run # 빌드와 실행을 하는 올인원 커맨드
$ cargo check # 코드가 컴파일 될 수 있는지 빠르게 체크하지만, 결과물은 만들지 않는다.
$ cargo update # 프로젝트 내 crate 버전 업데이트, lock 파일을 무시하고 'cago.toml' 내 버전으로 lock 파일을 최신화한다.
$ cargo doc [--open] # 프로젝트 내 포함된 의존성 문서를 만든다.
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
println!("Hello world!")
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

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

### Result

-   `enumeration`으로 `OK`와 `Err`의 값을 지니며 `expect` 메서드를 가지고 있음.
-   `expect`는 Result의 값이 `Err`일 때 인자로 건넨 메시지를 출력하고 프로그램이 예외를 발생시키도록함(Crash?).

### enumeration

-   각 값을 `variant`라고 부른다.
-   타입을 구현하는 개념인지 메서드가 존재한다.

### match

-   패턴 매칭표현식
-   `arm`으로 이루어짐
-   `match` `[enumreation을 리턴하는 표현식]` {`arm 목록`}의 형식으로 이루어진다.

```rust
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
```

#### arm

-   패턴과 패턴 일치 시 실행될 코드로 이루어짐

```rust
let guess = 12;
let secret_number = 14;
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
}
```

> 1. `cmp`가 `Ordering::Less`를 리턴
> 2. `match` 표현식이 `Ordering::Less` 값을 가지고 각 `arm`의 패턴을 체크
> 3. 첫번째 `arm`의 패턴과 일치하므로 "Too small!" 출력

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

> `_`는 catchall 값.

이렇게 무척 편리하게 사용할 수 있다.

### 문자열 보간

-   표현식을 사용할 때는 `{}`처럼 비워둔다.

```rust
let x = 5;
let y = 5;
println!("x is {x}");
println!("x = {} and y = {}", x, y)
```

### range expression

-   start..=end(inclusive)

```rust
let ran_num = rand::thread_rng().gen_range(1..=100);
```
