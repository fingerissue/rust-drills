// SOLUTION - 4.7 에러 핸들링 (Result/Option 처리, 에러 위임, overflow, 커스텀 에러)
use std::fmt;

// [문제 1] Result 기본 처리 - match로 성공/실패 각각 다른 문자열 반환
fn describe_result(r: Result<i32, String>) -> String {
    match r {
        Ok(v) => format!("성공: {}", v),
        Err(e) => format!("실패: {}", e),
    }
}

// [문제 2] ? 연산자로 에러 위임 - 파일 없이 문자열 파싱 두 단계 연쇄
fn parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError> {
    let n: i32 = s.parse()?;
    Ok(n * 2)
}

// [문제 3] overflow 처리 - checked_add로 오버플로우 감지
fn safe_add_u8(a: u8, b: u8) -> Option<u8> {
    a.checked_add(b)
}

// [문제 4] overflow 처리 - saturating_add로 최댓값에서 멈추기
fn saturating_add_u8(a: u8, b: u8) -> u8 {
    a.saturating_add(b)
}

// [문제 5] 커스텀 에러 타입 - std::error::Error 트레잇까지 구현
#[derive(Debug)]
struct NegativeAgeError(i32);

impl fmt::Display for NegativeAgeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "나이는 음수가 될 수 없습니다: {}", self.0)
    }
}
impl std::error::Error for NegativeAgeError {}

fn validate_age(age: i32) -> Result<u8, NegativeAgeError> {
    if age < 0 {
        Err(NegativeAgeError(age))
    } else {
        Ok(age as u8)
    }
}

// [문제 6] Result 처리 원칙 - unwrap_or_else로 기본값 제공하며 복구
fn get_config_value(key: &str) -> i32 {
    let table: std::collections::HashMap<&str, i32> =
        [("timeout", 30), ("retries", 3)].into_iter().collect();
    table.get(key).copied().unwrap_or_else(|| 0)
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
