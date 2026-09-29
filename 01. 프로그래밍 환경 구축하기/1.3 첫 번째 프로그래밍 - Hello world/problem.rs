// 1.3 첫 번째 프로그래밍: Hello world
// 실행: rustc problem.rs -o problem && ./problem
//
// [문제] cargo new 로 프로젝트를 만들면 src/main.rs 에 자동 생성되는
// 그 유명한 "Hello, world!" 를 반환하는 함수를 완성해라.
fn hello_message() -> String {
    return "Hello, world!".to_string();
}

fn main() {
    println!("{}", hello_message());
    assert_eq!(hello_message(), "Hello, world!");
    println!("✅ 통과");
}
