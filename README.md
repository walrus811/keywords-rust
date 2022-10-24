From [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html).


## [도구](#tool)

## [문법](#syntax)

<a href="tool"></a>

## 도구

### rustc
- 컴파일러

```
$ rustc --version
```

### rustup
- Rust 도구

```
$ rustup update
$ rustup self uninstall
$ rustup doc
```

### cargo
- 패키지 매니저.
- 시맨틱 버전 사용.
- [crates.io](https://crates.io/)

```
$ cargo --version
$ cargo [프로젝트 이름]
$ cargo build
$ cargo run
$ cargo check
```

### toml
- 컨피그 파일 형식(https://toml.io/en/)

<a href="syntax"></a>

## 문법

### 매크로
- 이름 뒤에 `!`가 붙음

```rust
println!("Hello world!")
```

### 변수
- `let`, 기본적으로 불변
- `mut`, 사용시 가변

```rust
let apples = 5;
let mut banana= 10;
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

### 변수의 섀도잉
- 같은 스코프 내에서 동일한 변수를 선언할 경우, 이전 변수가 가려짐.
- 변수를 다른 타입으로 바꾸는 등의 응용에 사용.

```rust
let mut guess = String::new();
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```