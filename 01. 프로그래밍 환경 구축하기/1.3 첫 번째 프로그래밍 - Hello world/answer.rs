// 1.3 첫 번째 프로그래밍: Hello world - 정답
fn hello_message() -> String {
    "Hello, world!".to_string()
}

fn main() {
    println!("{}", hello_message());
    assert_eq!(hello_message(), "Hello, world!");
    println!("✅ 통과");
}
