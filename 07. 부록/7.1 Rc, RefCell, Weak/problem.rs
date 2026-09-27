// Chapter 7.1 Rc<T>, RefCell<T>, Weak<T> (부록)
// 실행: rustc problem.rs -o problem && ./problem
// 자세한 개념은 같은 폴더의 개념설명.md 참고
use std::cell::RefCell;
use std::rc::{Rc, Weak};

// [문제 1] Rc - 값 42를 Rc로 감싸고, clone해서 참조 카운트를 2로 만든 뒤
// (a, 그 시점의 strong_count)를 반환해라
fn share_and_count() -> (Rc<i32>, usize) {
    todo!()
}

// [문제 2] RefCell - &self만으로 내부 값을 증가시키는 "내부 가변성" 구현
struct Counter {
    value: RefCell<i32>,
}
impl Counter {
    fn new() -> Self {
        Counter { value: RefCell::new(0) }
    }
    fn increment(&self) {
        todo!()
    }
    fn get(&self) -> i32 {
        todo!()
    }
}

// [문제 3] Rc<RefCell<T>> 조합 - 여러 소유자가 같은 데이터를 공유하며 수정
type SharedCounter = Rc<RefCell<i32>>;
fn make_shared_counter() -> SharedCounter {
    todo!()
}
fn increment_shared(counter: &SharedCounter) {
    todo!()
}

// [문제 4] Weak - 부모->자식은 Rc, 자식->부모는 Weak로 잡아서 순환참조 방지
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn build_parent_child() -> (Rc<Node>, Rc<Node>) {
    let parent = Rc::new(Node {
        value: 1,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    let child = Rc::new(Node {
        value: 2,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    // TODO: child.parent에 parent를 Weak로 연결하고,
    // parent.children에 child를 Rc로 추가해라
    todo!()
}

// [문제 5] Weak::upgrade로 부모 값을 안전하게 꺼내기 (부모가 없으면 None)
fn child_parent_value(child: &Rc<Node>) -> Option<i32> {
    todo!()
}

fn main() {
    let (a, count_at_clone) = share_and_count();
    assert_eq!(*a, 42);
    assert_eq!(count_at_clone, 2);
    println!("✅ 문제1 Rc(공유 소유권+참조 카운트) 통과");

    let counter = Counter::new();
    counter.increment();
    counter.increment();
    counter.increment();
    assert_eq!(counter.get(), 3);
    println!("✅ 문제2 RefCell(내부 가변성) 통과");

    let shared = make_shared_counter();
    let shared2 = Rc::clone(&shared);
    increment_shared(&shared);
    increment_shared(&shared2);
    assert_eq!(*shared.borrow(), 2);
    println!("✅ 문제3 Rc<RefCell<T>> 조합 통과");

    let (parent, child) = build_parent_child();
    assert_eq!(child_parent_value(&child), Some(1));
    assert_eq!(parent.children.borrow().len(), 1);
    println!("✅ 문제4,5 Weak(부모-자식, 순환참조 방지) 통과");

    println!("🎉 7.1 Rc/RefCell/Weak 챕터 완료!");
}
