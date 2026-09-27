// Chapter 7.4 클로저 심화 (Fn, FnMut, FnOnce, move) - 부록
// 실행: rustc problem.rs -o problem && ./problem
// 자세한 개념은 같은 폴더의 개념설명.md 참고

// [문제 1] Fn - 클로저를 두 번 적용 (f(f(x)))
fn call_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    todo!()
}

// [문제 2] FnMut - 캡처한 변수를 바꾸는 클로저를 n번 호출하고 마지막 결과 반환
fn apply_n_times<F: FnMut() -> i32>(mut f: F, n: u32) -> i32 {
    todo!()
}

// [문제 3] FnOnce - 클로저를 딱 한 번 호출해서 String을 얻고 길이 반환
fn consume_and_get_len<F: FnOnce() -> String>(f: F) -> usize {
    todo!()
}

// [문제 4] move 클로저 - n을 소유권째로 캡처해서, n을 더하는 함수를 반환
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    todo!()
}

// [문제 5] Box<dyn Fn> - 문자열에 따라 다른 연산 클로저를 반환
fn choose_operation(op: &str) -> Box<dyn Fn(i32, i32) -> i32> {
    todo!()
}

fn main() {
    assert_eq!(call_twice(|x| x * 2, 3), 12);
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
