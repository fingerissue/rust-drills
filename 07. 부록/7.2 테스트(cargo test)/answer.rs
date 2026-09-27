// SOLUTION - 7.2 테스트 (#[test], cargo test)
// 실행: rustc --test answer.rs -o answer_test && ./answer_test
// (일반 rustc가 아니라 --test 플래그로 컴파일해야 테스트 러너가 생긴다)
//
// 지금까지 모든 문제는 main() 안에 assert_eq!를 박아놓고 실행하는 방식이었는데,
// 실제 Rust 프로젝트에서는 #[test] 함수 + cargo test가 표준이다.

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("0으로 나눌 수 없음");
    }
    a / b
}

#[cfg(test)]
mod tests {
    use super::*;

    // [문제 1] 이미 작성된 예시 - add를 테스트하는 가장 기본적인 형태
    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    // [문제 2] is_even에 대한 테스트를 직접 작성 - 짝수/홀수 둘 다 확인
    #[test]
    fn test_is_even() {
        assert!(is_even(4));
        assert!(!is_even(7));
    }

    // [문제 3] #[should_panic] - divide(1, 0)이 패닉나는 걸 확인하는 테스트
    #[test]
    #[should_panic(expected = "0으로 나눌 수 없음")]
    fn test_divide_by_zero_panics() {
        divide(1, 0);
    }

    // [문제 4] #[ignore] - 평소엔 건너뛰고 `cargo test -- --ignored`로만 도는 느린 테스트 흉내
    #[test]
    #[ignore]
    fn test_slow_case() {
        assert_eq!(add(1_000_000, 1), 1_000_001);
    }
}
