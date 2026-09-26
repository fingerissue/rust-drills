// SOLUTION - 4.3 열거형(enum), Option, Result

#[derive(Debug, PartialEq)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle { base: f64, height: f64 },
}

// [문제 1] match로 각 도형의 넓이 계산
fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rectangle(w, h) => w * h,
        Shape::Triangle { base, height } => base * height / 2.0,
    }
}

// [문제 2] Option - 벡터에서 첫 번째 짝수 찾기
fn first_even(v: &[i32]) -> Option<i32> {
    v.iter().find(|&&x| x % 2 == 0).copied()
}

// [문제 3] Option 조합 - map으로 값이 있으면 변환, 없으면 그대로 None
fn double_if_some(x: Option<i32>) -> Option<i32> {
    x.map(|n| n * 2)
}

// [문제 4] Result 만들기 - 0으로 나누면 Err
fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("0으로 나눌 수 없습니다".to_string())
    } else {
        Ok(a / b)
    }
}

// [문제 5] ? 연산자로 에러 위임 (Result 처리 원칙)
fn calculate(a: f64, b: f64, c: f64) -> Result<f64, String> {
    let step1 = safe_divide(a, b)?;
    let step2 = safe_divide(step1, c)?;
    Ok(step2)
}

// [문제 6] Option -> Result 변환 (ok_or)
fn find_or_error(v: &[i32], target: i32) -> Result<usize, String> {
    v.iter().position(|&x| x == target).ok_or(format!("{}를 찾을 수 없습니다", target))
}

fn main() {
    assert!((area(&Shape::Circle(2.0)) - 12.566370614).abs() < 1e-6);
    assert_eq!(area(&Shape::Rectangle(3.0, 4.0)), 12.0);
    assert_eq!(area(&Shape::Triangle { base: 4.0, height: 5.0 }), 10.0);
    println!("✅ 문제1 area 통과");

    assert_eq!(first_even(&[1, 3, 4, 5]), Some(4));
    assert_eq!(first_even(&[1, 3, 5]), None);
    println!("✅ 문제2 first_even 통과");

    assert_eq!(double_if_some(Some(5)), Some(10));
    assert_eq!(double_if_some(None), None);
    println!("✅ 문제3 double_if_some 통과");

    assert_eq!(safe_divide(10.0, 2.0), Ok(5.0));
    assert!(safe_divide(10.0, 0.0).is_err());
    println!("✅ 문제4 safe_divide 통과");

    assert_eq!(calculate(100.0, 5.0, 2.0), Ok(10.0));
    assert!(calculate(100.0, 0.0, 2.0).is_err());
    println!("✅ 문제5 calculate(?연산자) 통과");

    assert_eq!(find_or_error(&[1, 2, 3], 2), Ok(1));
    assert!(find_or_error(&[1, 2, 3], 9).is_err());
    println!("✅ 문제6 find_or_error(ok_or) 통과");

    println!("🎉 4.3 열거형/Option/Result 챕터 완료!");
}
