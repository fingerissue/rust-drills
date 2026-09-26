// Chapter 4.9 기타 (?Sized, Deref, 타입 별칭, Peekable, statement/expression)
// 실행: rustc problem.rs -o problem && ./problem
use std::ops::Deref;

// [문제 1] ?Sized - 크기가 불확정인 타입도 받을 수 있는 제네릭 함수 (Debug 포맷으로 문자열 반환)
fn print_len<T: ?Sized + std::fmt::Debug>(_x: &T) -> String {
    todo!()
}

// [문제 2] Deref - 커스텀 스마트 포인터가 안의 값을 자동으로 역참조하게 만들기
struct MyBox<T>(T);
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        todo!()
    }
}

fn take_str_len(s: &str) -> usize {
    s.len()
}

// [문제 3] 타입 별칭 (Type Alias) - 복잡한 타입에 이름 붙이고 값들의 합 구하기
type Score = i32;
type ScoreTable = std::collections::HashMap<String, Score>;

fn total_score(table: &ScoreTable) -> Score {
    todo!()
}

// [문제 4] Peekable - 다음 값을 미리 보고, 값이 감소하기 시작하면 멈추면서 지금까지 센 개수 반환
fn count_until_decrease(v: &[i32]) -> usize {
    let mut iter = v.iter().peekable();
    let mut count = 0;
    todo!()
}

// [문제 5] statement(구문) vs expression(표현) - if 블록 자체를 표현식으로 사용
fn classify(n: i32) -> &'static str {
    todo!()
}

fn main() {
    assert_eq!(print_len("hi"), "\"hi\"");
    println!("✅ 문제1 print_len(?Sized) 통과");

    let boxed = MyBox(String::from("hello deref"));
    assert_eq!(take_str_len(&boxed), 11);
    println!("✅ 문제2 MyBox(Deref) 통과");

    let mut table: ScoreTable = ScoreTable::new();
    table.insert("치준환".to_string(), 90);
    table.insert("동료".to_string(), 80);
    assert_eq!(total_score(&table), 170);
    println!("✅ 문제3 ScoreTable(타입 별칭) 통과");

    assert_eq!(count_until_decrease(&[1, 3, 5, 2, 8]), 3);
    println!("✅ 문제4 count_until_decrease(Peekable) 통과");

    assert_eq!(classify(4), "짝수");
    assert_eq!(classify(3), "홀수");
    println!("✅ 문제5 classify(statement vs expression) 통과");

    println!("🎉 4.9 기타 챕터 완료!");
}
