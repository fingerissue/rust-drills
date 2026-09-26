// SOLUTION - 3.1 메인 함수와 화면 출력 (println!, print!, format!, 포맷 옵션)

// [문제 1] 기본 출력 - 이름과 나이를 넣어 "이름: {}, 나이: {}" 형태의 문자열 만들기
fn basic_greeting(name: &str, age: u8) -> String {
    format!("이름: {}, 나이: {}", name, age)
}

// [문제 2] 디버그 포맷 {:?} - 벡터를 디버그 포맷으로 문자열화
fn format_vec_debug(v: &[i32]) -> String {
    format!("{:?}", v)
}

// [문제 3] 소수점 자리수 지정 {:.2} - 가격을 소수점 둘째 자리까지 표시
fn format_price(amount: f64) -> String {
    format!("가격: {:.2}원", amount)
}

// [문제 4] 정렬/폭 지정 - 왼쪽 정렬 10칸(<10) + 오른쪽 정렬 5칸(>5)
fn format_table_row(name: &str, score: i32) -> String {
    format!("{:<10}{:>5}", name, score)
}

// [문제 5] 이름 붙은 인자(named argument) 인라인 문법
fn format_intro(name: &str, age: u8) -> String {
    format!("저는 {name}이고 {age}살이에요")
}

// [개념 체크] print!는 개행이 없고 println!은 개행이 있다.
// 아래처럼 여러 번 print!를 호출하면 한 줄로 이어붙듯 출력된다.
// print!("A"); print!("B"); println!("C"); // 출력: ABC (줄바꿈은 마지막에만)
// 이 문제는 코드로 검증하기보다 개념만 알고 넘어가면 된다.

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
