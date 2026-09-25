// 2.2 작성한 코드 이해하기 - 정답
// 2.1에서 배운 내용(main 함수, println! 매크로, for 루프)을 응용해서
// while 루프 버전 + println!의 인라인 변수 문법({sum} 형태)을 연습한다.
fn main() {
    let result = get_product(5);
    println!("1*...*5={result}"); // {sum} 처럼 중괄호 안에 변수명을 직접 써도 된다.
}

fn get_product(n: u32) -> u32 {
    let mut product: u32 = 1;
    let mut i: u32 = 1;

    while i <= n {
        product *= i;
        i += 1;
    }

    product
}
