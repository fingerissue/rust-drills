// Chapter 7.2 테스트 (#[test], cargo test) - 부록
// 실행: rustc --test problem.rs -o problem_test && ./problem_test
// (일반 rustc가 아니라 --test 플래그로 컴파일해야 테스트 러너가 생긴다!)
// 자세한 개념은 같은 폴더의 개념설명.md 참고

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

    // [문제 2] is_even에 대한 테스트 완성하기 - 짝수/홀수 둘 다 확인
    #[test]
    fn test_is_even() {
        todo!()
    }

    // [문제 3] #[should_panic] - divide(1, 0)이 패닉나는 걸 확인 (이미 붙어있는 어트리뷰트 그대로 사용)
    #[test]
    #[should_panic(expected = "0으로 나눌 수 없음")]
    fn test_divide_by_zero_panics() {
        todo!()
    }

    // [문제 4] #[ignore] - 평소엔 건너뛰고 `--ignored` 옵션으로만 도는 테스트 완성하기
    #[test]
    #[ignore]
    fn test_slow_case() {
        todo!()
    }
}
