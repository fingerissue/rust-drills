// 2.1 무작정 따라하며 Rust 코드 짜 보기(20분)
// 실행: rustc problem.rs -o problem && ./problem
//
// 책에서 그대로 타이핑해보라고 한 예제다. 1부터 n까지의 합을 구하는 함수를 만들고,
// main에서 get_sum(100)을 호출해서 "1+...+100=5050" 형태로 출력해라.

fn main() {
    println!("1+...+100={}", get_sum(100));
}

fn get_sum(n: u32) -> u32 {
    // TODO: mut 변수 sum을 0으로 선언하고,
    // for 루프(1..=n)를 돌면서 sum에 i를 더한 뒤 반환해라.
    todo!()
}
