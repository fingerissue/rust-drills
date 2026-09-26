// Chapter 4.2 구조체 (정의/생성, 튜플 구조체, 메서드, 갱신 문법, 비어있는 구조체)
// 실행: rustc problem.rs -o problem && ./problem

#[derive(Debug, PartialEq)]
struct User {
    name: String,
    age: u8,
    active: bool,
}

// [문제 1] 생성자 함수 - active는 항상 true로 시작
fn new_user(name: &str, age: u8) -> User {
    todo!()
}

// [문제 2] 구조체 갱신 문법(..u)으로 이름만 바꾼 새 User 만들기
fn rename_user(u: User, new_name: &str) -> User {
    todo!()
}

// [문제 3] 튜플 구조체 - 원점으로부터의 거리
struct Point3D(f64, f64, f64);
fn distance_from_origin(p: &Point3D) -> f64 {
    todo!()
}

impl User {
    // [문제 4] 나이를 1살 늘리는 메서드
    fn have_birthday(&mut self) {
        todo!()
    }
    // [문제 5] 활성 상태를 뒤집는 메서드
    fn toggle_active(&mut self) {
        todo!()
    }
}

// [문제 6] 비어있는 구조체(유닛 구조체) - 마커 타입으로 트레잇 구현에 활용
struct AdminRole;
trait RoleName {
    fn role_name(&self) -> &'static str;
}
impl RoleName for AdminRole {
    fn role_name(&self) -> &'static str {
        todo!()
    }
}

fn main() {
    let u = new_user("치준환", 20);
    assert_eq!(u, User { name: "치준환".to_string(), age: 20, active: true });
    println!("✅ 문제1 new_user 통과");

    let u2 = rename_user(u, "새이름");
    assert_eq!(u2.name, "새이름");
    assert_eq!(u2.age, 20);
    println!("✅ 문제2 rename_user 통과");

    let p = Point3D(3.0, 4.0, 0.0);
    assert_eq!(distance_from_origin(&p), 5.0);
    println!("✅ 문제3 Point3D(튜플 구조체) 통과");

    let mut u3 = new_user("test", 19);
    u3.have_birthday();
    assert_eq!(u3.age, 20);
    println!("✅ 문제4 have_birthday 통과");

    u3.toggle_active();
    assert_eq!(u3.active, false);
    println!("✅ 문제5 toggle_active 통과");

    assert_eq!(AdminRole.role_name(), "admin");
    println!("✅ 문제6 AdminRole(유닛 구조체) 통과");

    println!("🎉 4.2 구조체 챕터 완료!");
}
