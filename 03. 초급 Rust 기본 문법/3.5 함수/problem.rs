// 3.5 함수, 메서드, 매크로, 클로저
// 실행: rustc problem.rs -o problem && ./problem

// [문제 1] 두 수 중 큰 값을 반환하는 함수
fn max_of(a: i32, b: i32) -> i32 {
    todo!()
}

// [문제 2] 메서드 - Rectangle의 넓이(width * height)를 구하는 메서드
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        todo!()
    }
}

// [문제 3] 매크로 - macro_rules!로 숫자를 제곱하는 square!(x) 매크로 만들기
macro_rules! square {
    ($x:expr) => {
        todo!()
    };
}

// [문제 4] 클로저 - 정수를 받아서 제곱해 반환하는 클로저를 만들어 반환하는 함수
fn make_squarer() -> impl Fn(i32) -> i32 {
    todo!()
}

// [문제 5] 클로저를 인자로 받아 두 번 적용하는 함수 (f(f(x)))
fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    todo!()
}

// [문제 6] move 클로저 - base 값을 캡처해서 x+base를 계산하는 클로저 반환
fn make_adder(base: i32) -> impl Fn(i32) -> i32 {
    todo!()
}

fn main() {
    assert_eq!(max_of(3, 7), 7);
    println!("✅ 문제1 max_of 통과");

    let rect = Rectangle { width: 3, height: 4 };
    assert_eq!(rect.area(), 12);
    println!("✅ 문제2 Rectangle::area 통과");

    assert_eq!(square!(5), 25);
    println!("✅ 문제3 square!(매크로) 통과");

    let square = make_squarer();
    assert_eq!(square(5), 25);
    println!("✅ 문제4 make_squarer(클로저 반환) 통과");

    assert_eq!(apply_twice(|x| x + 3, 1), 7);
    println!("✅ 문제5 apply_twice 통과");

    let add5 = make_adder(5);
    assert_eq!(add5(10), 15);
    println!("✅ 문제6 make_adder(move 클로저) 통과");

    println!("🎉 3.5 함수/메서드/매크로/클로저 챕터 완료!");
}
