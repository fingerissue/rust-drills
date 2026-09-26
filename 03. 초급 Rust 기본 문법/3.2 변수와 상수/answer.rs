// SOLUTION - 3.2 변수와 상수 (let, mut, shadowing, const)

// [문제 1] shadowing - x에 5를 넣고 그 값+1로 shadowing
fn shadow_example() -> i32 {
    let x = 5;
    let x = x + 1;
    x
}

// [문제 2] shadowing으로 타입 바꾸기 - 공백 개수를 세는 문자열을 정수로 변환
fn shadow_type_change(spaces: &str) -> usize {
    let spaces = spaces.len(); // &str -> usize로 shadowing
    spaces
}

// [문제 3] mut - 카운터를 n번 증가시키기
fn increment_n_times(start: i32, n: u32) -> i32 {
    let mut count = start;
    for _ in 0..n {
        count += 1;
    }
    count
}

// [문제 4] const - 원의 넓이를 구하는데 PI는 상수로 선언
fn circle_area(radius: f64) -> f64 {
    const PI: f64 = 3.14159;
    PI * radius * radius
}

// [문제 5] 튜플 구조 분해로 변수 여러 개 한 번에 선언
fn sum_and_diff(a: i32, b: i32) -> (i32, i32) {
    let (sum, diff) = (a + b, a - b);
    (sum, diff)
}

fn main() {
    assert_eq!(shadow_example(), 6);
    println!("✅ 문제1 shadow_example 통과");

    assert_eq!(shadow_type_change("   "), 3);
    println!("✅ 문제2 shadow_type_change 통과");

    assert_eq!(increment_n_times(0, 5), 5);
    assert_eq!(increment_n_times(10, 3), 13);
    println!("✅ 문제3 increment_n_times 통과");

    assert!((circle_area(2.0) - 12.56636).abs() < 1e-4);
    println!("✅ 문제4 circle_area(const) 통과");

    assert_eq!(sum_and_diff(5, 3), (8, 2));
    println!("✅ 문제5 sum_and_diff(튜플 구조분해) 통과");

    println!("🎉 3.2 변수와 상수 챕터 완료!");
}
