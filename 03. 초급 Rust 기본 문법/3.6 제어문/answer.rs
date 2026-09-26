// SOLUTION - 3.6 제어문 (if, match, if let, for, loop, while)

// [문제 1] if/else 체인 - 점수를 등급으로 변환
fn grade(score: i32) -> char {
    if score >= 90 { 'A' }
    else if score >= 80 { 'B' }
    else if score >= 70 { 'C' }
    else { 'F' }
}

// [문제 2] match - Option<i32>을 받아 값이 있으면 2배, 없으면 0 반환
fn double_or_zero(x: Option<i32>) -> i32 {
    match x {
        Some(n) => n * 2,
        None => 0,
    }
}

// [문제 3] if let - Option에서 값이 있을 때만 1을 더해서 반환, 없으면 -1
fn add_one_if_some(x: Option<i32>) -> i32 {
    if let Some(n) = x {
        n + 1
    } else {
        -1
    }
}

// [문제 4] for - 1부터 n까지의 합
fn sum_to_n(n: u32) -> u32 {
    let mut total = 0;
    for i in 1..=n {
        total += i;
    }
    total
}

// [문제 5] loop + break with value - 3의 배수 중 처음으로 20보다 큰 값 찾기
fn first_multiple_of_3_over_20() -> i32 {
    let mut n = 0;
    loop {
        n += 3;
        if n > 20 {
            break n;
        }
    }
}

// [문제 6] while - 카운트다운 값들을 벡터로 모으기 (n부터 1까지)
fn countdown(n: u32) -> Vec<u32> {
    let mut result = Vec::new();
    let mut i = n;
    while i >= 1 {
        result.push(i);
        i -= 1;
    }
    result
}

fn main() {
    assert_eq!(grade(95), 'A');
    assert_eq!(grade(60), 'F');
    println!("✅ 문제1 grade 통과");

    assert_eq!(double_or_zero(Some(5)), 10);
    assert_eq!(double_or_zero(None), 0);
    println!("✅ 문제2 double_or_zero 통과");

    assert_eq!(add_one_if_some(Some(4)), 5);
    assert_eq!(add_one_if_some(None), -1);
    println!("✅ 문제3 add_one_if_some(if let) 통과");

    assert_eq!(sum_to_n(10), 55);
    println!("✅ 문제4 sum_to_n 통과");

    assert_eq!(first_multiple_of_3_over_20(), 21);
    println!("✅ 문제5 first_multiple_of_3_over_20 통과");

    assert_eq!(countdown(3), vec![3, 2, 1]);
    println!("✅ 문제6 countdown(while) 통과");

    println!("🎉 3.6 제어문 챕터 완료!");
}
