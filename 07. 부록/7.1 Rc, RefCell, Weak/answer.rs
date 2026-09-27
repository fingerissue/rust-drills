// SOLUTION - 7.1 Rc<T>, RefCell<T>, Weak<T>
use std::cell::RefCell;
use std::rc::{Rc, Weak};

// [문제 1] Rc - 같은 데이터를 두 곳에서 공유하고, 참조 카운트 확인
fn share_and_count() -> (Rc<i32>, usize) {
    let a = Rc::new(42);
    let b = Rc::clone(&a); // 데이터 복사 아님 - 참조 카운트만 +1
    let count = Rc::strong_count(&a);
    drop(b);
    (a, count) // clone 직후의 count(2)를 반환
}

// [문제 2] RefCell - 불변 참조(&self)만 있어도 내부 값을 바꾸는 "내부 가변성"
struct Counter {
    value: RefCell<i32>,
}
impl Counter {
    fn new() -> Self {
        Counter { value: RefCell::new(0) }
    }
    fn increment(&self) {
        // &self인데도 내부 값을 바꿀 수 있는 게 RefCell의 핵심
        *self.value.borrow_mut() += 1;
    }
    fn get(&self) -> i32 {
        *self.value.borrow()
    }
}

// [문제 3] Rc<RefCell<T>> 조합 - 여러 곳에서 공유하면서 다 같이 수정 가능한 데이터
type SharedCounter = Rc<RefCell<i32>>;
fn make_shared_counter() -> SharedCounter {
    Rc::new(RefCell::new(0))
}
fn increment_shared(counter: &SharedCounter) {
    *counter.borrow_mut() += 1;
}

// [문제 4] Weak - 부모/자식 트리에서 순환 참조 방지
// 부모->자식은 Rc(강한 참조), 자식->부모는 Weak(약한 참조)로 잡아야 메모리가 안 샌다.
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
    *child.parent.borrow_mut() = Rc::downgrade(&parent);
    parent.children.borrow_mut().push(Rc::clone(&child));
    (parent, child)
}

fn child_parent_value(child: &Rc<Node>) -> Option<i32> {
    child.parent.borrow().upgrade().map(|p| p.value)
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
    assert_eq!(*shared.borrow(), 2); // 둘 다 같은 데이터를 봄
    println!("✅ 문제3 Rc<RefCell<T>> 조합 통과");

    let (parent, child) = build_parent_child();
    assert_eq!(child_parent_value(&child), Some(1));
    assert_eq!(parent.children.borrow().len(), 1);
    println!("✅ 문제4 Weak(부모-자식, 순환참조 방지) 통과");

    println!("🎉 7.1 Rc/RefCell/Weak 챕터 완료!");
}
