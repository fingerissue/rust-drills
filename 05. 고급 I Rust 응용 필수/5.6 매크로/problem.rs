// Chapter 5.6 매크로 (선언적 매크로, macro_rules!)
// 실행: rustc problem.rs -o problem && ./problem

// [문제 1] 가장 단순한 매크로 - 인자 없이 고정 문자열("안녕, 매크로!") 반환
macro_rules! greeting {
    () => {
        todo!()
    };
}

// [문제 2] 인자 하나를 받는 매크로 - 제곱을 계산하는 표현식으로 확장
macro_rules! square {
    ($x:expr) => {
        todo!()
    };
}

// [문제 3] 가변 인자 매크로 - vec! 처럼 여러 개의 값을 받아서 Vec으로 만들기
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut v = Vec::new();
            // TODO: $x 들을 순서대로 v에 push
            v
        }
    };
}

// [문제 4] 여러 패턴을 매칭하는 매크로 - 인자 개수에 따라 다르게 동작(재귀)
macro_rules! add_all {
    () => { 0 };
    ($x:expr) => { $x };
    ($x:expr, $($rest:expr),+) => {
        // TODO: $x 와 나머지(add_all!($($rest),+))를 더해서 반환
        todo!()
    };
}

fn main() {
    assert_eq!(greeting!(), "안녕, 매크로!");
    println!("✅ 문제1 greeting!() 통과");

    assert_eq!(square!(5), 25);
    println!("✅ 문제2 square!(x) 통과");

    let v: Vec<i32> = my_vec![1, 2, 3, 4];
    assert_eq!(v, vec![1, 2, 3, 4]);
    println!("✅ 문제3 my_vec!(가변 인자) 통과");

    assert_eq!(add_all!(1, 2, 3, 4), 10);
    assert_eq!(add_all!(), 0);
    println!("✅ 문제4 add_all!(재귀 매크로) 통과");

    println!("🎉 5.6 매크로 챕터 완료!");
}
