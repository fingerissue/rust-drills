// SOLUTION - 3.9 반복자 (Iterator: map, filter, filter_map, take_while, enumerate, zip)

// [문제 1] map - 모든 원소를 제곱
fn square_all(v: &[i32]) -> Vec<i32> {
    v.iter().map(|x| x * x).collect()
}

// [문제 2] filter + map 체이닝 - 짝수만 골라서 2배
fn double_evens(v: &[i32]) -> Vec<i32> {
    v.iter().filter(|&&x| x % 2 == 0).map(|x| x * 2).collect()
}

// [문제 3] filter_map - 문자열 중 숫자로 파싱 가능한 것만 파싱해서 합산
fn sum_parsable(v: &[&str]) -> i32 {
    v.iter().filter_map(|s| s.parse::<i32>().ok()).sum()
}

// [문제 4] take_while - 정렬된 벡터에서 5보다 작은 값만 앞에서부터 가져오기
fn take_less_than_5(v: &[i32]) -> Vec<i32> {
    v.iter().take_while(|&&x| x < 5).cloned().collect()
}

// [문제 5] enumerate - 인덱스가 짝수인 원소들만 골라내기
fn elements_at_even_index(v: &[i32]) -> Vec<i32> {
    v.iter().enumerate().filter(|(i, _)| i % 2 == 0).map(|(_, &x)| x).collect()
}

// [문제 6] zip - 두 벡터를 짝지어 합산한 벡터 만들기
fn zip_sum(a: &[i32], b: &[i32]) -> Vec<i32> {
    a.iter().zip(b.iter()).map(|(x, y)| x + y).collect()
}

fn main() {
    assert_eq!(square_all(&[1, 2, 3]), vec![1, 4, 9]);
    println!("✅ 문제1 square_all 통과");

    assert_eq!(double_evens(&[1, 2, 3, 4]), vec![4, 8]);
    println!("✅ 문제2 double_evens 통과");

    assert_eq!(sum_parsable(&["1", "abc", "3", "x", "5"]), 9);
    println!("✅ 문제3 sum_parsable 통과");

    assert_eq!(take_less_than_5(&[1, 3, 4, 7, 2]), vec![1, 3, 4]);
    println!("✅ 문제4 take_less_than_5 통과");

    assert_eq!(elements_at_even_index(&[10, 20, 30, 40, 50]), vec![10, 30, 50]);
    println!("✅ 문제5 elements_at_even_index(enumerate) 통과");

    assert_eq!(zip_sum(&[1, 2, 3], &[10, 20, 30]), vec![11, 22, 33]);
    println!("✅ 문제6 zip_sum 통과");

    println!("🎉 3.9 반복자 챕터 완료!");
}
