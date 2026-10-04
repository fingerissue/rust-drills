// 3.3 데이터 타입 (스칼라: 정수/부동소수점/불리언/문자, 복합: 튜플/배열, 디폴트 값)
// 실행: rustc problem.rs -o problem && ./problem

// [문제 1] 정수 타입 캐스팅(as 키워드) - i32 두 개를 f64로 변환해서 평균 구하기
fn average(a: i32, b: i32) -> f64 {
    let a: f64 = a as f64;
    let b: f64 = b as f64;
    (a + b) / 2.0
}

// [문제 2] char 메서드 - 알파벳 대문자인지 판별 (is_ascii_uppercase 활용)
fn is_upper_alpha(c: char) -> bool {
    c.is_ascii_uppercase()
}

// [문제 3] 튜플 인덱싱 - (이름, 나이, 키) 튜플에서 나이(.1)만 뽑아 반환
fn get_age(person: (&str, u8, f32)) -> u8 {
    person.1
}

// [문제 4] 배열 슬라이싱 - [i32; 5] 배열의 앞 3개 원소 합
fn sum_first_three(arr: [i32; 5]) -> i32 {
    arr[..3].iter().sum()
}

// [문제 5] 디폴트 값 - Default 트레잇으로 기본값을 만들고 retries 필드만 원하는 값으로 바꾸기
#[derive(Debug, Default, PartialEq)]
struct Config {
    verbose: bool,
    retries: u8,
    name: String,
}
fn default_config_with_retries(retries: u8) -> Config {
    Config{ retries, ..Default::default() }
}

// [문제 6] 불리언 로직 - VIP거나(||) 구매금액이 10만원 이상이면 할인 대상
fn is_eligible_for_discount(is_vip: bool, amount: u32) -> bool {
    is_vip || amount >= 100000
}

fn main() {
    assert_eq!(average(3, 4), 3.5);
    println!("✅ 문제1 average(as 캐스팅) 통과");

    assert_eq!(is_upper_alpha('A'), true);
    assert_eq!(is_upper_alpha('a'), false);
    println!("✅ 문제2 is_upper_alpha 통과");

    assert_eq!(get_age(("치준환", 20, 175.5)), 20);
    println!("✅ 문제3 get_age(튜플 인덱싱) 통과");

    assert_eq!(sum_first_three([1, 2, 3, 4, 5]), 6);
    println!("✅ 문제4 sum_first_three(슬라이싱) 통과");

    let cfg = default_config_with_retries(3);
    assert_eq!(cfg, Config { verbose: false, retries: 3, name: String::new() });
    println!("✅ 문제5 default_config_with_retries(Default 트레잇) 통과");

    assert_eq!(is_eligible_for_discount(true, 0), true);
    assert_eq!(is_eligible_for_discount(false, 150_000), true);
    assert_eq!(is_eligible_for_discount(false, 5000), false);
    println!("✅ 문제6 is_eligible_for_discount 통과");

    println!("🎉 3.3 데이터 타입 챕터 완료!");
}
