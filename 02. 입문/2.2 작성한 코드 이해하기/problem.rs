// 2.2 작성한 코드 이해하기 (20분)
// 실행: rustc problem.rs -o problem && ./problem
//
// 2.1에서는 for 루프 + println!("...{}", 변수) 형태를 썼다.
// 이번엔 같은 결과를 while 루프로 만들고, println!의 인라인 변수 문법
// (예: println!("a+b={sum}");)을 사용해서 출력해라.
//
// [문제] 1부터 n까지의 곱을 구하는 get_product 함수를 while 루프로 작성하고,
// main에서 get_product(5) 결과를 "1*...*5=120" 형태로 출력해라. (인라인 변수 문법 사용)

fn main() {
    let result = get_product(5);
    // TODO: println! 인라인 변수 문법으로 "1*...*5={result}" 출력
    println!("1*...*5={result}");
}

fn get_product(n: u32) -> u32 {
    // TODO: mut 변수 product를 1로, i를 1로 선언하고
    // while i <= n 조건으로 product *= i, i += 1 을 반복한 뒤 product 반환
    let mut product: u32 = 1;
    let mut i: u32 = 1;
    
    while i <= n {
        product *= i;
        i += 1;
    }

    return product;
}
