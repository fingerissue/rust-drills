// Chapter 4.7 에러 핸들링 (Result/Option 처리, 에러 위임, overflow, 커스텀 에러)
// 실행: rustc problem.rs -o problem && ./problem
// 참고: thiserror/anyhow 크레이트는 외부 의존성이 필요해서 이 파일에서는 다루지 않음.
//       필요하면 cargo 프로젝트로 별도 실습 예정.
use std::fmt;

// [문제 1] Result 기본 처리 - match로 성공/실패 각각 다른 문자열 반환
fn describe_result(r: Result<i32, String>) -> String {
    todo!()
}

// [문제 2] ? 연산자로 에러 위임 - 문자열을 파싱해서 2배로
fn parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError> {
    todo!()
}

// [문제 3] overflow 처리 - checked_add로 오버플로우 감지 (u8 기준)
fn safe_add_u8(a: u8, b: u8) -> Option<u8> {
    todo!()
}

// [문제 4] overflow 처리 - saturating_add로 최댓값(255)에서 멈추기
fn saturating_add_u8(a: u8, b: u8) -> u8 {
    todo!()
}

// [문제 5] 커스텀 에러 타입 - Display와 std::error::Error 구현
#[derive(Debug)]
struct NegativeAgeError(i32);

impl fmt::Display for NegativeAgeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: "나이는 음수가 될 수 없습니다: {}" 형태로 self.0 을 포함해 작성
        todo!()
    }
}
impl std::error::Error for NegativeAgeError {}

fn validate_age(age: i32) -> Result<u8, NegativeAgeError> {
    todo!()
}

// [문제 6] Result/Option 처리 원칙 - unwrap_or_else로 기본값 제공
fn get_config_value(key: &str) -> i32 {
    let table: std::collections::HashMap<&str, i32> =
        [("timeout", 30), ("retries", 3)].into_iter().collect();
    todo!()
}

fn main() {
    assert_eq!(describe_result(Ok(5)), "성공: 5");
    assert_eq!(describe_result(Err("실패했음".to_string())), "실패: 실패했음");
    println!("✅ 문제1 describe_result 통과");

    assert_eq!(parse_and_double("21").unwrap(), 42);
    assert!(parse_and_double("abc").is_err());
    println!("✅ 문제2 parse_and_double(? 위임) 통과");

    assert_eq!(safe_add_u8(200, 50), Some(250));
    assert_eq!(safe_add_u8(200, 100), None);
    println!("✅ 문제3 safe_add_u8(checked_add) 통과");

    assert_eq!(saturating_add_u8(200, 100), 255);
    println!("✅ 문제4 saturating_add_u8 통과");

    assert_eq!(validate_age(20).unwrap(), 20);
    assert!(validate_age(-5).is_err());
    assert_eq!(validate_age(-5).unwrap_err().to_string(), "나이는 음수가 될 수 없습니다: -5");
    println!("✅ 문제5 NegativeAgeError(커스텀 에러) 통과");

    assert_eq!(get_config_value("timeout"), 30);
    assert_eq!(get_config_value("unknown"), 0);
    println!("✅ 문제6 get_config_value(unwrap_or_else) 통과");

    println!("🎉 4.7 에러 핸들링 챕터 완료!");
}
