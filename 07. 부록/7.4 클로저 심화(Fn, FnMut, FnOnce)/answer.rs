// SOLUTION - 7.4 클로저 심화 (Fn, FnMut, FnOnce, move)

// [문제 1] Fn - 아무것도 변경하지 않고 그냥 읽기만 하는 클로저 (여러 번 호출 가능)
fn call_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(f(x))
}

// [문제 2] FnMut - 클로저 내부에서 캡처한 변수를 변경하는 클로저 (여러 번 호출 가능)
fn apply_n_times<F: FnMut() -> i32>(mut f: F, n: u32) -> i32 {
    let mut last = 0;
    for _ in 0..n {
        last = f();
    }
    last
}

// [문제 3] FnOnce - 캡처한 값의 소유권을 가져가서 한 번만 호출 가능한 클로저
fn consume_and_get_len<F: FnOnce() -> String>(f: F) -> usize {
    let s = f();
    s.len()
}

// [문제 4] move 클로저 - 클로저가 캡처한 변수의 소유권을 강제로 가져가게 만들기
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n // move 없으면 n의 참조만 캡처되는데, 함수를 반환하려면 소유권이 필요함
}

// [문제 5] Box<dyn Fn> - 클로저를 함수의 리턴값/필드로 저장하기 (트레잇 객체)
fn choose_operation(op: &str) -> Box<dyn Fn(i32, i32) -> i32> {
    match op {
        "+" => Box::new(|a, b| a + b),
        "*" => Box::new(|a, b| a * b),
        _ => Box::new(|_, _| 0),
    }
}

fn main() {
    assert_eq!(call_twice(|x| x * 2, 3), 12); // (3*2)*2
    println!("✅ 문제1 Fn(call_twice) 통과");

    let mut count = 0;
    let counter = || {
        count += 1;
        count
    };
    assert_eq!(apply_n_times(counter, 5), 5);
    println!("✅ 문제2 FnMut(apply_n_times) 통과");

    let owned = String::from("hello world");
    assert_eq!(consume_and_get_len(move || owned), 11);
    println!("✅ 문제3 FnOnce(consume_and_get_len) 통과");

    let add5 = make_adder(5);
    assert_eq!(add5(10), 15);
    println!("✅ 문제4 move 클로저(make_adder) 통과");

    let add_op = choose_operation("+");
    let mul_op = choose_operation("*");
    assert_eq!(add_op(3, 4), 7);
    assert_eq!(mul_op(3, 4), 12);
    println!("✅ 문제5 Box<dyn Fn>(choose_operation) 통과");

    println!("🎉 7.4 클로저 심화 챕터 완료!");
}
