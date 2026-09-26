use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

// Chapter 3.7 컬렉션 (Vec, HashMap, HashSet, VecDeque)
// 실행: rustc problem.rs -o problem && ./problem

// [문제 1] Vec - 짝수만 걸러서 새 Vec으로 반환
fn even_numbers(v: &[i32]) -> Vec<i32> {
    todo!()
}

// [문제 2] Vec - pop()으로 마지막 3개를 제거한 벡터를 반환
fn drop_last_three(mut v: Vec<i32>) -> Vec<i32> {
    todo!()
}

// [문제 3] HashMap - entry API로 단어별 등장 횟수 세기
fn word_count(words: &[&str]) -> HashMap<String, i32> {
    todo!()
}

// [문제 4] HashMap - 이름으로 점수 조회, 없으면 0 반환 (unwrap_or 활용)
fn get_score(scores: &HashMap<String, i32>, name: &str) -> i32 {
    todo!()
}

// [문제 5] HashSet - 두 벡터의 교집합 개수
fn intersection_count(a: &[i32], b: &[i32]) -> usize {
    todo!()
}

// [문제 6] VecDeque - 짝수 인덱스는 뒤(push_back), 홀수 인덱스는 앞(push_front)에 넣기
fn process_queue(items: &[i32]) -> VecDeque<i32> {
    todo!()
}

fn main() {
    assert_eq!(even_numbers(&[1, 2, 3, 4, 5, 6]), vec![2, 4, 6]);
    println!("✅ 문제1 even_numbers 통과");

    assert_eq!(drop_last_three(vec![1, 2, 3, 4, 5]), vec![1, 2]);
    println!("✅ 문제2 drop_last_three 통과");

    let counts = word_count(&["apple", "banana", "apple"]);
    assert_eq!(counts.get("apple"), Some(&2));
    assert_eq!(counts.get("banana"), Some(&1));
    println!("✅ 문제3 word_count 통과");

    let mut scores = HashMap::new();
    scores.insert("치준환".to_string(), 90);
    assert_eq!(get_score(&scores, "치준환"), 90);
    assert_eq!(get_score(&scores, "없는사람"), 0);
    println!("✅ 문제4 get_score 통과");

    assert_eq!(intersection_count(&[1, 2, 3], &[2, 3, 4]), 2);
    println!("✅ 문제5 intersection_count 통과");

    let result = process_queue(&[1, 2, 3, 4, 5]);
    assert_eq!(result, VecDeque::from(vec![4, 2, 1, 3, 5]));
    println!("✅ 문제6 process_queue(VecDeque) 통과");

    println!("🎉 3.7 컬렉션 챕터 완료!");
}
