// 3.1 메인 함수와 화면 출력
// 실행: rustc problem.rs -o problem && ./problem
// format!, println!, print! 매크로와 각종 포맷 옵션을 연습한다.

// [문제 1] format!으로 "이름: {}, 나이: {}" 형태의 문자열 만들기
fn basic_greeting(name: &str, age: u8) -> String {
    todo!()
}

// [문제 2] 디버그 포맷 {:?} - 벡터를 디버그 포맷 문자열로 ("[1, 2, 3]" 형태)
fn format_vec_debug(v: &[i32]) -> String {
    todo!()
}

// [문제 3] 소수점 자리수 지정 {:.2} - "가격: {:.2}원" 형태 (1234.5 -> "가격: 1234.50원")
fn format_price(amount: f64) -> String {
    todo!()
}

// [문제 4] 정렬/폭 지정 - 이름은 왼쪽 정렬 10칸({:<10}), 점수는 오른쪽 정렬 5칸({:>5})
fn format_table_row(name: &str, score: i32) -> String {
    todo!()
}

// [문제 5] 이름 붙은 인자(named argument) 인라인 문법 사용 - "저는 {name}이고 {age}살이에요"
fn format_intro(name: &str, age: u8) -> String {
    todo!()
}

// [개념 체크] print!는 개행이 없고 println!은 개행이 있다.
// print!("A"); print!("B"); println!("C"); 를 실행하면 어떻게 출력될지 예상해보자.
// (정답: ABC 한 줄로 출력된다 - 줄바꿈은 println! 호출 시점에만 생긴다)

fn main() {
    assert_eq!(basic_greeting("치준환", 20), "이름: 치준환, 나이: 20");
    println!("✅ 문제1 basic_greeting 통과");

    assert_eq!(format_vec_debug(&[1, 2, 3]), "[1, 2, 3]");
    println!("✅ 문제2 format_vec_debug 통과");

    assert_eq!(format_price(1234.5), "가격: 1234.50원");
    println!("✅ 문제3 format_price 통과");

    assert_eq!(format_table_row("치준환", 95), "치준환          95");
    println!("✅ 문제4 format_table_row 통과");

    assert_eq!(format_intro("치준환", 20), "저는 치준환이고 20살이에요");
    println!("✅ 문제5 format_intro(named arg) 통과");

    println!("🎉 3.1 메인 함수와 화면 출력 챕터 완료!");
}
