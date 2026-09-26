use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

// SOLUTION - 3.7 컬렉션 (Vec, HashMap, HashSet, VecDeque)

// [문제 1] Vec - 짝수만 걸러서 새 Vec으로 반환
fn even_numbers(v: &[i32]) -> Vec<i32> {
    v.iter().filter(|&&x| x % 2 == 0).cloned().collect()
}

// [문제 2] Vec - push/pop으로 스택처럼 사용, 마지막 3개 제거 후 반환
fn drop_last_three(mut v: Vec<i32>) -> Vec<i32> {
    for _ in 0..3 {
        v.pop();
    }
    v
}

// [문제 3] HashMap - 단어별 등장 횟수 세기 (entry API 사용)
fn word_count(words: &[&str]) -> HashMap<String, i32> {
    let mut map = HashMap::new();
    for w in words {
        *map.entry(w.to_string()).or_insert(0) += 1;
    }
    map
}

// [문제 4] HashMap - 학생 이름으로 점수 조회, 없으면 0점 처리
fn get_score(scores: &HashMap<String, i32>, name: &str) -> i32 {
    *scores.get(name).unwrap_or(&0)
}

// [문제 5] HashSet - 두 벡터의 교집합 개수
fn intersection_count(a: &[i32], b: &[i32]) -> usize {
    let set_a: HashSet<_> = a.iter().collect();
    let set_b: HashSet<_> = b.iter().collect();
    set_a.intersection(&set_b).count()
}

// [문제 6] VecDeque - 앞/뒤 양쪽에서 넣고 빼는 큐
fn process_queue(items: &[i32]) -> VecDeque<i32> {
    let mut dq: VecDeque<i32> = VecDeque::new();
    for (i, &item) in items.iter().enumerate() {
        if i % 2 == 0 {
            dq.push_back(item);
        } else {
            dq.push_front(item);
        }
    }
    dq
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

    // items = [1,2,3,4,5] -> i=0(back:1) i=1(front:2) i=2(back:3) i=3(front:4) i=4(back:5)
    // front push 순서 반대로 쌓임: [4,2] + [1,3,5] = [4,2,1,3,5]
    let result = process_queue(&[1, 2, 3, 4, 5]);
    assert_eq!(result, VecDeque::from(vec![4, 2, 1, 3, 5]));
    println!("✅ 문제6 process_queue(VecDeque) 통과");

    println!("🎉 3.7 컬렉션 챕터 완료!");
}
