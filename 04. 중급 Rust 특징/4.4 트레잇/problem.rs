// Chapter 4.4 트레잇 (개념, 항목, 구현, derive, 바운드, 연관 타입, 다형성)
// 실행: rustc problem.rs -o problem && ./problem

trait Speak {
    fn sound(&self) -> String;
    fn introduce(&self) -> String {
        format!("나는 {}라고 말해요", self.sound())
    }
}

// [문제 1] 이 구조체가 Clone 되고(==)로 비교도 가능하도록 derive 어트리뷰트를 추가해라
// TODO: #[derive(...)] 를 여기에 추가
struct Dog {
    name: String,
}
struct Cat;

// [문제 1-b] Dog는 "멍멍", Cat은 "야옹"을 반환하도록 트레잇 구현
impl Speak for Dog {
    fn sound(&self) -> String {
        todo!()
    }
}
impl Speak for Cat {
    fn sound(&self) -> String {
        todo!()
    }
}

// [문제 2] 트레잇 바운드 - 제네릭 함수에서 introduce() 호출
fn print_intro<T: Speak>(animal: &T) -> String {
    todo!()
}

// [문제 3] 연관 타입(associated type) - 인덱스로 값을 꺼내는 커스텀 트레잇
trait Container {
    type Item;
    fn get(&self, i: usize) -> Option<&Self::Item>;
}
struct Bag(Vec<i32>);
impl Container for Bag {
    type Item = i32;
    fn get(&self, i: usize) -> Option<&i32> {
        todo!()
    }
}

// [문제 4] 정적 바인딩 (impl Trait) - Dog를 반환
fn make_dog() -> impl Speak {
    todo!()
}

// [문제 5] 동적 바인딩 (Box<dyn Trait>) - 여러 타입의 sound()를 모으기
fn all_sounds(animals: &[Box<dyn Speak>]) -> Vec<String> {
    todo!()
}

// [문제 6] where 절로 트레잇 바운드 걸기 - 둘 중 sound() 문자열이 더 긴 쪽을 반환
fn longest_sound<T>(a: &T, b: &T) -> String
where
    T: Speak,
{
    todo!()
}

fn main() {
    let d1 = Dog { name: "초코".to_string() };
    let d2 = d1.clone();
    assert_eq!(d1, d2);
    println!("✅ 문제1 derive(Clone, PartialEq) 통과");

    assert_eq!(print_intro(&Dog { name: "초코".to_string() }), "나는 멍멍라고 말해요");
    println!("✅ 문제2 print_intro 통과");

    let bag = Bag(vec![10, 20, 30]);
    assert_eq!(bag.get(1), Some(&20));
    println!("✅ 문제3 연관 타입(Container) 통과");

    let d = make_dog();
    assert_eq!(d.sound(), "멍멍");
    println!("✅ 문제4 make_dog(impl Trait) 통과");

    let animals: Vec<Box<dyn Speak>> = vec![Box::new(Dog { name: "a".into() }), Box::new(Cat)];
    assert_eq!(all_sounds(&animals), vec!["멍멍", "야옹"]);
    println!("✅ 문제5 all_sounds(Box<dyn Trait>) 통과");

    assert_eq!(longest_sound(&Cat, &Cat), "야옹");
    println!("✅ 문제6 longest_sound(where절) 통과");

    println!("🎉 4.4 트레잇 챕터 완료!");
}
