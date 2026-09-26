// SOLUTION - 4.1 소유권 (Ownership), 대여(Borrowing), Copy/Clone

// [문제 1] 소유권 이동 - String을 넘기면 소유권이 이동한다. 참조로 받아서 길이 반환
fn string_length(s: &String) -> usize {
    s.len()
}

// [문제 2] 가변 대여 - 벡터의 모든 원소에 10을 더하기
fn add_ten_to_all(v: &mut Vec<i32>) {
    for x in v.iter_mut() {
        *x += 10;
    }
}

// [문제 3] 대여 규칙 - 같은 벡터에서 최댓값의 인덱스를 찾기 (불변 참조만 사용)
fn max_index(v: &[i32]) -> usize {
    let mut max_i = 0;
    for i in 1..v.len() {
        if v[i] > v[max_i] {
            max_i = i;
        }
    }
    max_i
}

// [문제 4] Clone - 구조체를 복제해서 원본은 그대로, 복제본만 수정
#[derive(Clone, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
fn move_right(p: &Point, dx: i32) -> Point {
    let mut cloned = p.clone();
    cloned.x += dx;
    cloned
}

// [문제 5] Copy - i32는 Copy 트레잇이 있어서 대입해도 원본을 계속 쓸 수 있다
fn add_copy(a: i32, b: i32) -> i32 {
    let c = a; // Copy라서 a는 여전히 유효
    c + a + b - a
}

// [문제 6] 소유권 반환 - 벡터를 받아서 소유권째로 정렬해서 반환 (move 후 재사용 패턴)
fn take_and_sort(mut v: Vec<i32>) -> Vec<i32> {
    v.sort();
    v
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
