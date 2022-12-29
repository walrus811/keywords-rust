From [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html).

- ### [도구](#tool)

- ### [문법](#syntax)

<a id="tool"></a>

## 도구

### rustup

- Rust 도구

```
$ rustup update
$ rustup self uninstall
$ rustup doc
```

### rustc

- 컴파일러
- AOT 컴파일
- 언어 구조상 컴파일 타임에 레퍼런스나 동시성 문제를 잡아낼 수 있음

```
$ rustc --version
```

### cargo

- 빌드 시스템이자 패키지 매니저.
- 시맨틱 버전 사용.
- [crates.io](https://crates.io/)
- 실행 커맨드 동작 시 소스의 변화를 감지하여 필요한 순간에만 빌드한다.

```
$ cargo --version
$ cargo new [프로젝트 이름]
$ cargo build [--release]
$ cargo run # 빌드와 실행을 하는 올인원 커맨드
$ cargo check # 코드가 컴파일 될 수 있는지 빠르게 체크하지만, 결과물은 만들지 않는다.
$ cargo update # 프로젝트 내 crate 버전 업데이트
```

#### cargo 프로젝트 구조

- 소스 코드는 `src`에 보관
- 루트 디렉토리에는 README, 라이센스, 설정 파일 등 보관

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

- 컨피그 파일 형식(https://toml.io/en/)

### 컨벤션

- 소스 파일 이름은 `snake_case`로 짓는다.

### Crates.io

- [패키지 저장소](https://crates.io/)

<a id="syntax"></a>

## 문법

### 매크로

- 이름 뒤에 `!`가 붙음

```rust
println!("Hello world!")
```

### indent

- 탭이 아니라 스페이스 4개를 씀

### prelude

- 표준 라이브러리 중 Rust가 자동으로 임포트하는 것들
- 굳이 임포트할 필요는 없다.
- [목록](https://doc.rust-lang.org/std/prelude/index.html)

### 변수

- `let` 키워드로 변수를 선언한다. 기본적으로 불변. 컴파일 타임에 안전한 코드를 작성할 수 있게 해준다.
- `mut`, 이름 앞에 사용, 사용시 가변. 명시적인 사용으로 이 변수를 다른 코드가 바꿀 수 있다는 걸 암시한다.
- 컨벤션으로 `snake_case`를 사용한다.

```rust
let apples = 5;
let mut banana= 10;
```

### 상수

- `const` 키워드를 쓴다.
- 타입이 항상 명시 되어야한다.
- 스코프 상관 없이 쓰일 수 있다.
- 컴파일 타임에 값이 결정가능한 표현식만 온다.
- 자신이 속한 스코프 내에서 프로그램이 끝날 때까지 유효하다.
- `mut`와 쓸 수 없다. 상수는 변경 될 수 있다 개념이 없기 때문이다.
- 컨벤션으로 `UPPER_CASE`를 사용한다.

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

### 변수의 섀도잉

- 같은 스코프 내에서 동일한 변수를 선언할 경우, 이전 변수가 가려짐.
- `let`과 함께 동일한 이름을 쓴다.
- 변수를 다른 타입으로 바꾸는 등의 응용에 사용.
- 굳이 이름을 안 바꿔도 된다.
- `mut`와는 여러 차이점이 존재한다.
  - `let` 키워드를 사용하는 덕에 다른 스코프에서 해당 변수를 사용하고 변경하고도 변수의 불변성을 유지할 수 있다(컴파일 타임 에러를 얻을 수 있다).
  - 섀도잉은 변수의 타입을 변경하는 응용이 가능하다.

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

- Rust는 정적 타입 언어다.
- 스칼라(Scalar) 타입과 컴파운드(Compound) 타입이 존재한다.

#### 스칼라 타입

- 단일 값을 나타낸다.

1. 정수(integer)

- `i8`, `u8`, `i16`, `u16`, `i32`, `u32`, `i64`, `u64`, `i128`, `u128`, `isize`, `usize`가 존재한다.
- `isize`와 `usize`는 머신의 레지스터 사이즈에 의존한다.
- 디폴트는 `i32`.
- `_`은 숫자 리터럴을 읽기 쉽게 구분하는데 쓰임(`98_222`).
- 16진수(`0x`), 8진수(`0o`), 2진수(`0b`).
- 바이트(`b'A'`) - **u8 타입**
- 오버플로우시에 다음 두 동작 중 한가지가 벌어진다.
  - 디버그 모드시, Rust는 런타임에서 오버플로우 발생시 프로그램을 `panic` 상태로 만드는 체크 로직을 포함한다.
  - 릴리즈 모드시, Rust는 위 같은 체크로직을 포함하지 않는다. 대신 `2의 보수 래핑`이 벌어진다. 이 동작은 정수의 범위 내에서 값이 회전하는 동작을 의미한다. 가령, u8에서 256은 0이 되고, 257은 1이 되는 식이다. 즉, 프로그램이 `panic`에 빠지지 않는다.
- 오버플로우를 다루기 위해 `wrapping_*`, `checked_*`, `overflowing_*`, `saturating_*` 메서드들을 활용한다.

> `panic`은 프로그램이 에러와 함께 종료 되었을 때를 일컫는다.

2. 실수(flating-point)

- `f32`,`f64`가 존재한다.
- 디폴트는 `f64`.
- 리터럴을 사용할 때 `f32`는 타입을 명시해주어야 한다.

3. 불린(boolean)

- `bool` 키워드를 사용한다.
- `true`,`false` 값이 존재한다.

4. 문자(character)

- `char` 키워드를 사용한다.
- 유니코드를 표현하기 위해 4바이트 크기를 가진다.
- `'`를 사용한다.

#### 컴파운드 타입

- 한 타입에 여러 값이 들어가 있는 타입이다.

1. 튜플(Tuple)

- 이종의 데이터를 모아놓은 컴파운드 타입.
- 고정 크기를 가진다.
- 패턴 매칭으로 `destructuring` 동작이 가능하다.
- `.` 뒤에 인덱스를 붙여 단일 값을 조회할 수도 있다.
- 아무런 값도 지니지 않은 특수한 튜플을 유닛(unit)이라고 한다. 값과 타입 그 자체가 `()`로 표현된다. 빈 값이나 빈 리턴 타입을 표현할 때 사용 된다.
- 표현식이 아무것도 리턴하지 않으면 유닛이 리턴 된다.

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;
let five_hundred = x.0;
let six_point_four = x.1;
let one = x.2;
```

2. 배열(Array)

- 동종의 데이터를 모아놓은 컴파운드 타입.
- 고정 길이.
- 데이터를 힙이 아니라 스택에 할당하고 싶을 때 유용하다.
- 동적 배열로는 표준 라이브러리에 `vector`가 있다.
- 같은 값을 여러번 할당하고 싶을 땐 `[값; 갯수]` 표현식을 사용한다.
- 인덱스를 벗어나는 접근은 `panic`을 발생시킨다. 이는 릴리즈 모드에서도 동일하다. 이는 다른 시스템 프로그램 언어들이 지니는 메모리 오염 문제를 막기 위해서다.

```rust
let a = [1, 2, 3, 4, 5];
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
let a: [i32; 5] = [1, 2, 3, 4, 5];
let a = [3; 5];
let a = [3, 3, 3, 3, 3];
```

### 함수

- `fn` 키워드를 사용해 정의한다.
- `main`은 특수한 함수로 프로그램 엔트리 포인트로 쓰인다.
- 컨벤션으로 `snake_case`를 사용한다.
- 함수 정의 순서나 위치는 신경 쓰지 않아도 된다.
- 매크로를 호출하는 것은 표현식이다.
- 리턴 타입은 `->`로 표시한다.
- Rust에서 함수의 리턴 값과 \**블록의 마지막 표현식의 값*을 동일하다.
- 물론 `return` 키워드도 존재하지만 이는 앞서서 함수를 종료시킬 때 사용한다.
- 대부분의 경우, 이 마지막 표현식을 암시적인 리턴값으로 사용한다.
- 마지막 표현식에는 `;`이 붙지 않는다.
- 아무것도 리턴하지 않으면 `()`, 유닛이 리턴 된다.
- 마지막 표현식에 `;`가 붙으면 아무것도 리턴하지 않는 게 되어버린다.

```rust
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1 // ;이 없다. 이 표현식이 평가 된다.
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

- `//`만 쓴다.

### if 표현식

- `if` 표현식은 `arm`이라고도 불린다. `arm`은 `if` 표현식과 그 실행 코드를 함께 일컫는 말이다.
- 조건은 반드신 `bool`이어야만 한다(`if 3` 같은 건 안 된다).
- `else if`가 두 개가 넘어간다면 `match`를 고려해 볼 것.
- `if`는 표현식이므로 `let`과도 같이 쓰일 수 있다. 이 경우, 모든 표현식이 같은 타입으로 평가 되어야 한다.

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

- `loop`,`while`,`for`가 존재한다.
- `loop`는 명시적인 중단이 있을 때까지 계속 반복한다.
- `loop`는 `break`를 통해 값을 리턴할 수도 있다.
- `loop`에 레이블을 붙일 수도 있다. 이를 통해 중첩된 구조의 빠른 탈출이 가능하다.
- 특정 인덱스나 갯수만큼 반복하는 것도 `Range`와 `for`를 활용하는 게 좋다..

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

### 레퍼런스

### Result

- `enumeration`으로 `OK`와 `Err`의 값을 지니며 `expect` 메서드를 가지고 있음.
- `expect`는 Result의 값이 `Err`일 때 인자로 건넨 메시지를 출력하고 프로그램이 예외를 발생시키도록함(Crash?).

### enumeration

- 각 값을 `variant`라고 부른다.
- 타입을 구현하는 개념인지 메서드가 존재한다.

### match

- 패턴 매칭
- `arm`으로 이루어짐
- `match` `[enumreation을 리턴하는 표현식]` {`arm 목록`}의 형식으로 이루어진다.

#### arm

- 패턴과 패턴 일치 시 실행될 코드로 이루어짐

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

```rust
let x = 5;
let y = 5;
println!("x is {x}");
println!("x = {} and y = {}", x, y)
```

### range expression

- start..=end(inclusive)

```rust
let ran_num = rand::thread_rng().gen_range(1..=100);
```
