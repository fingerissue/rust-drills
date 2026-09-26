// 3.2 변수와 상수 (let, mut, shadowing, const)
// 실행: rustc problem.rs -o problem && ./problem

// [문제 1] shadowing - x에 5를 넣고 그 값+1로 shadowing해서 반환
fn shadow_example() -> i32 {
    todo!()
}

// [문제 2] shadowing으로 타입 바꾸기 - &str을 받아서 길이(usize)로 shadowing
fn shadow_type_change(spaces: &str) -> usize {
    todo!()
}

// [문제 3] mut - start에서 시작해서 n번 1씩 증가시킨 값 반환
fn increment_n_times(start: i32, n: u32) -> i32 {
    todo!()
}

// [문제 4] const - 함수 내부에 PI를 const로 선언하고 원의 넓이(PI*r*r) 계산
fn circle_area(radius: f64) -> f64 {
    todo!()
}

// [문제 5] 튜플 구조 분해로 합과 차를 한 번에 계산해서 (합, 차) 튜플로 반환
fn sum_and_diff(a: i32, b: i32) -> (i32, i32) {
    todo!()
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
