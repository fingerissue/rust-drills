// Chapter 4.1 소유권(Ownership), 대여(Borrowing), Copy/Clone
// 실행: rustc problem.rs -o problem && ./problem

// [문제 1] String의 길이를 반환 (소유권을 가져오지 않고 참조로 받기)
fn string_length(s: &String) -> usize {
    todo!()
}

// [문제 2] 가변 참조로 벡터의 모든 원소에 10을 더하기
fn add_ten_to_all(v: &mut Vec<i32>) {
    todo!()
}

// [문제 3] 불변 참조만으로 벡터에서 최댓값의 인덱스를 찾기
fn max_index(v: &[i32]) -> usize {
    todo!()
}

// [문제 4] Clone - 구조체를 복제해서 원본은 그대로 두고 복제본의 x만 dx만큼 이동
#[derive(Clone, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
fn move_right(p: &Point, dx: i32) -> Point {
    todo!()
}

// [문제 5] Copy - i32는 Copy라서 이동 없이 복사된다는 걸 이용해 a를 여러 번 쓰는 계산 (c+a+b-a)
fn add_copy(a: i32, b: i32) -> i32 {
    let c = a;
    todo!()
}

// [문제 6] 소유권을 넘겨받아서(move) 정렬한 뒤 그대로 반환
fn take_and_sort(mut v: Vec<i32>) -> Vec<i32> {
    todo!()
}

fn main() {
    let s = String::from("hello");
    assert_eq!(string_length(&s), 5);
    assert_eq!(s, "hello");
    println!("✅ 문제1 string_length 통과");

    let mut v = vec![1, 2, 3];
    add_ten_to_all(&mut v);
    assert_eq!(v, vec![11, 12, 13]);
    println!("✅ 문제2 add_ten_to_all 통과");

    assert_eq!(max_index(&[3, 7, 2, 9, 4]), 3);
    println!("✅ 문제3 max_index 통과");

    let p1 = Point { x: 0, y: 0 };
    let p2 = move_right(&p1, 5);
    assert_eq!(p1, Point { x: 0, y: 0 });
    assert_eq!(p2, Point { x: 5, y: 0 });
    println!("✅ 문제4 Clone(move_right) 통과");

    assert_eq!(add_copy(3, 4), 7);
    println!("✅ 문제5 Copy(add_copy) 통과");

    assert_eq!(take_and_sort(vec![3, 1, 2]), vec![1, 2, 3]);
    println!("✅ 문제6 take_and_sort 통과");

    println!("🎉 4.1 소유권/대여/Copy/Clone 챕터 완료!");
}
