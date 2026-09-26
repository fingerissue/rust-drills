// 3.6 제어문 (if, match, if let, for, loop, while)
// 실행: rustc problem.rs -o problem && ./problem

// [문제 1] 점수를 등급으로: 90이상 A, 80이상 B, 70이상 C, 그외 F
fn grade(score: i32) -> char {
    todo!()
}

// [문제 2] match - Option<i32>을 받아 값이 있으면 2배, 없으면 0 반환
fn double_or_zero(x: Option<i32>) -> i32 {
    todo!()
}

// [문제 3] if let - Option에서 값이 있을 때만 1을 더해서 반환, 없으면 -1
fn add_one_if_some(x: Option<i32>) -> i32 {
    todo!()
}

// [문제 4] for - 1부터 n까지의 합
fn sum_to_n(n: u32) -> u32 {
    todo!()
}

// [문제 5] loop + break with value - 3의 배수 중 처음으로 20보다 큰 값 찾기
fn first_multiple_of_3_over_20() -> i32 {
    todo!()
}

// [문제 6] while - n부터 1까지 카운트다운한 값들을 벡터로 모으기
fn countdown(n: u32) -> Vec<u32> {
    todo!()
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
