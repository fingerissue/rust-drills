// SOLUTION - 4.5 제네릭 (함수/구조체/열거형 적용, 라이프타임)

// [문제 1] 제네릭 함수 - 벡터에서 최댓값 찾기 (PartialOrd 바운드)
fn find_max<T: PartialOrd + Copy>(v: &[T]) -> T {
    let mut max = v[0];
    for &x in v.iter() {
        if x > max {
            max = x;
        }
    }
    max
}

// [문제 2] 제네릭 구조체
struct Pair<T> {
    first: T,
    second: T,
}
impl<T: std::ops::Add<Output = T> + Copy> Pair<T> {
    fn sum(&self) -> T {
        self.first + self.second
    }
}

// [문제 3] 제네릭 열거형 - 내 것과 남의 것 구분하는 소유 표시자
enum Owned<T> {
    Mine(T),
    Theirs(T),
}
fn unwrap_owned<T>(o: Owned<T>) -> T {
    match o {
        Owned::Mine(v) => v,
        Owned::Theirs(v) => v,
    }
}

// [문제 4] 라이프타임 표기 - 두 문자열 슬라이스 중 더 긴 것 반환
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

// [문제 5] 구조체에서의 라이프타임 표기 - 문자열을 소유하지 않고 참조만 담는 구조체
struct Excerpt<'a> {
    part: &'a str,
}
impl<'a> Excerpt<'a> {
    fn announce(&self) -> String {
        format!("주목: {}", self.part)
    }
}

// [문제 6] 여러 타입 파라미터를 갖는 제네릭 함수
fn combine<A: std::fmt::Display, B: std::fmt::Display>(a: A, b: B) -> String {
    format!("{}-{}", a, b)
}

fn main() {
    assert_eq!(find_max(&[3, 7, 2, 9, 4]), 9);
    assert_eq!(find_max(&[1.5, 2.5, 0.5]), 2.5);
    println!("✅ 문제1 find_max(제네릭 함수) 통과");

    let p = Pair { first: 3, second: 4 };
    assert_eq!(p.sum(), 7);
    println!("✅ 문제2 Pair(제네릭 구조체) 통과");

    assert_eq!(unwrap_owned(Owned::Mine(10)), 10);
    assert_eq!(unwrap_owned(Owned::Theirs("hi")), "hi");
    println!("✅ 문제3 Owned(제네릭 열거형) 통과");

    assert_eq!(longest("hello", "hi"), "hello");
    println!("✅ 문제4 longest(라이프타임) 통과");

    let text = String::from("치준환이 러스트를 공부한다");
    let first_word = text.split_whitespace().next().unwrap();
    let excerpt = Excerpt { part: first_word };
    assert_eq!(excerpt.announce(), "주목: 치준환이");
    println!("✅ 문제5 Excerpt(구조체 라이프타임) 통과");

    assert_eq!(combine(1, "a"), "1-a");
    println!("✅ 문제6 combine(다중 타입 파라미터) 통과");

    println!("🎉 4.5 제네릭/라이프타임 챕터 완료!");
}
