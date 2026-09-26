// SOLUTION - 4.4 트레잇 (개념, 항목, 구현, derive, 바운드, 연관 타입, 다형성)

trait Speak {
    fn sound(&self) -> String;
    // 트레잇 항목: 디폴트 메서드
    fn introduce(&self) -> String {
        format!("나는 {}라고 말해요", self.sound())
    }
}

#[derive(Debug, Clone, PartialEq)] // [문제 1] derive로 자동 구현
struct Dog {
    name: String,
}
struct Cat;

impl Speak for Dog {
    fn sound(&self) -> String {
        "멍멍".to_string()
    }
}
impl Speak for Cat {
    fn sound(&self) -> String {
        "야옹".to_string()
    }
}

// [문제 2] 트레잇 바운드 - 제네릭 함수에서 트레잇 사용
fn print_intro<T: Speak>(animal: &T) -> String {
    animal.introduce()
}

// [문제 3] 연관 타입 - Iterator 유사한 커스텀 트레잇
trait Container {
    type Item;
    fn get(&self, i: usize) -> Option<&Self::Item>;
}
struct Bag(Vec<i32>);
impl Container for Bag {
    type Item = i32;
    fn get(&self, i: usize) -> Option<&i32> {
        self.0.get(i)
    }
}

// [문제 4] 정적 바인딩 (impl Trait)
fn make_dog() -> impl Speak {
    Dog { name: "치준환의 개".to_string() }
}

// [문제 5] 동적 바인딩 (Box<dyn Trait>) - 여러 타입을 한 벡터에
fn all_sounds(animals: &[Box<dyn Speak>]) -> Vec<String> {
    animals.iter().map(|a| a.sound()).collect()
}

// [문제 6] 트레잇 바운드에 where 절 사용
fn longest_sound<T>(a: &T, b: &T) -> String
where
    T: Speak,
{
    let sa = a.sound();
    let sb = b.sound();
    if sa.len() >= sb.len() { sa } else { sb }
}

fn main() {
    let d1 = Dog { name: "초코".to_string() };
    let d2 = d1.clone(); // derive(Clone) 확인
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
