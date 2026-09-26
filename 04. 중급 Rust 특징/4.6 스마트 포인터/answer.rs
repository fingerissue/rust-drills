// SOLUTION - 4.6 스마트 포인터 (Box: 크기 한정, 동적 디스패치, 포인터 이동)

// [문제 1] 재귀 데이터 구조 - Box로 컴파일 타임에 크기를 한정 (연결 리스트)
#[derive(Debug, PartialEq)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
use List::{Cons, Nil};

fn list_sum(list: &List) -> i32 {
    match list {
        Cons(v, next) => v + list_sum(next),
        Nil => 0,
    }
}

fn make_list(values: &[i32]) -> List {
    let mut list = Nil;
    for &v in values.iter().rev() {
        list = Cons(v, Box::new(list));
    }
    list
}

// [문제 2] Box<dyn Trait> - 동적 디스패치로 여러 구현체를 하나의 벡터에
trait Shape {
    fn area(&self) -> f64;
}
struct Circle {
    r: f64,
}
struct Square {
    side: f64,
}
impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.r * self.r
    }
}
impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}
fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

// [문제 3] Box 안의 값을 꺼내서 사용 (역참조 *)
fn add_boxed(a: Box<i32>, b: Box<i32>) -> i32 {
    *a + *b
}

// [문제 4] Box move - Box를 다른 변수로 옮겨도(move) 힙 데이터는 그대로, 포인터만 이동
fn take_box(b: Box<String>) -> usize {
    // move된 b를 여기서 그대로 사용 (실제 데이터 복사 없이 소유권만 이동)
    b.len()
}

fn main() {
    let list = make_list(&[1, 2, 3, 4]);
    assert_eq!(list_sum(&list), 10);
    println!("✅ 문제1 재귀 리스트(Box<List>) 통과");

    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { r: 1.0 }),
        Box::new(Square { side: 2.0 }),
    ];
    assert!((total_area(&shapes) - (std::f64::consts::PI + 4.0)).abs() < 1e-9);
    println!("✅ 문제2 total_area(Box<dyn Trait>) 통과");

    assert_eq!(add_boxed(Box::new(3), Box::new(4)), 7);
    println!("✅ 문제3 add_boxed(역참조) 통과");

    let boxed_string = Box::new(String::from("hello box"));
    assert_eq!(take_box(boxed_string), 9);
    println!("✅ 문제4 take_box(Box move) 통과");

    println!("🎉 4.6 스마트 포인터(Box) 챕터 완료!");
}
