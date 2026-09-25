// 2.1 무작정 따라하며 Rust 코드 짜 보기 - 정답
// 책 원문 그대로: 1부터 n까지의 합을 구하는 함수를 만들고 호출해서 출력한다.
fn main() {
    println!("1+...+100={}", get_sum(100));
}

fn get_sum(n: u32) -> u32 {
    let mut sum: u32 = 0;

    for i in 1..=n {
        sum += i;
    }

    return sum;
}
