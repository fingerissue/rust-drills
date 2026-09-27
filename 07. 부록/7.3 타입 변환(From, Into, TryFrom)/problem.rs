// Chapter 7.3 타입 변환 (From, Into, TryFrom, TryInto) - 부록
// 실행: rustc problem.rs -o problem && ./problem
// 자세한 개념은 같은 폴더의 개념설명.md 참고

// [문제 1] From 구현 - Celsius -> Fahrenheit (공식: F = C * 9/5 + 32)
struct Celsius(f64);
struct Fahrenheit(f64);

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        todo!()
    }
}

// [문제 2] From을 구현하면 Into는 자동으로 딸려온다 - .into()로 변환해서 값 꺼내기
fn celsius_to_fahrenheit_value(c: Celsius) -> f64 {
    todo!()
}

// [문제 3] 여러 타입에서 하나의 커스텀 타입으로 변환 (From<i32>, From<&str>)
#[derive(Debug, PartialEq)]
struct Id(String);

impl From<i32> for Id {
    fn from(n: i32) -> Self {
        // "ID-{n}" 형태로 만들어라
        todo!()
    }
}
impl From<&str> for Id {
    fn from(s: &str) -> Self {
        todo!()
    }
}

// [문제 4] TryFrom - 실패할 수 있는 변환 (0~100 범위를 벗어나면 에러)
#[derive(Debug, PartialEq)]
struct Percentage(u8);

impl TryFrom<i32> for Percentage {
    type Error = String;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        todo!()
    }
}

// [문제 5] 함수 파라미터에서 Into<T> 바운드 사용 - 호출부가 더 유연해짐
fn describe_id(id: impl Into<Id>) -> String {
    todo!()
}

fn main() {
    let c = Celsius(100.0);
    let f = Fahrenheit::from(c);
    assert_eq!(f.0, 212.0);
    println!("✅ 문제1 From<Celsius> for Fahrenheit 통과");

    assert_eq!(celsius_to_fahrenheit_value(Celsius(0.0)), 32.0);
    println!("✅ 문제2 .into() 통과");

    assert_eq!(Id::from(42), Id("ID-42".to_string()));
    assert_eq!(Id::from("custom-id"), Id("custom-id".to_string()));
    println!("✅ 문제3 여러 타입에서 From 통과");

    assert_eq!(Percentage::try_from(50), Ok(Percentage(50)));
    assert!(Percentage::try_from(150).is_err());
    println!("✅ 문제4 TryFrom(실패 가능한 변환) 통과");

    assert_eq!(describe_id(7), "발급된 ID: ID-7");
    assert_eq!(describe_id("abc"), "발급된 ID: abc");
    println!("✅ 문제5 impl Into<T> 파라미터 통과");

    println!("🎉 7.3 타입 변환(From/Into/TryFrom) 챕터 완료!");
}
