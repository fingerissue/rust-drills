// Chapter 3.8 문자열 (&str, String)
// 실행: rustc problem.rs -o problem && ./problem

// [문제 1] "안녕하세요, {이름}!" 을 만드는 함수 (String 조작: push_str/push)
fn greet_name(name: &str) -> String {
    todo!()
}

// [문제 2] 문자열 뒤집기 (유니코드 char 단위)
fn reverse_string(s: &str) -> String {
    todo!()
}

// [문제 3] 공백 기준 단어 개수 세기
fn word_count(s: &str) -> usize {
    todo!()
}

// [문제 4] 특정 문자로 시작하는 단어만 추출
fn words_starting_with<'a>(s: &'a str, prefix: char) -> Vec<&'a str> {
    todo!()
}

// [문제 5] 앞에서부터 n글자만 잘라서 반환
fn truncate(s: &str, n: usize) -> String {
    todo!()
}

// [문제 6] trim + 대문자 변환
fn clean_and_upper(s: &str) -> String {
    todo!()
}

fn main() {
    assert_eq!(greet_name("치준환"), "안녕하세요, 치준환!");
    println!("✅ 문제1 greet_name 통과");

    assert_eq!(reverse_string("rust"), "tsur");
    assert_eq!(reverse_string("러스트"), "트스러");
    println!("✅ 문제2 reverse_string 통과");

    assert_eq!(word_count("나는 러스트를 배운다"), 3);
    println!("✅ 문제3 word_count 통과");

    assert_eq!(words_starting_with("apple ant banana avocado", 'a'), vec!["apple", "ant", "avocado"]);
    println!("✅ 문제4 words_starting_with 통과");

    assert_eq!(truncate("hello world", 5), "hello");
    println!("✅ 문제5 truncate 통과");

    assert_eq!(clean_and_upper("  hello rust  "), "HELLO RUST");
    println!("✅ 문제6 clean_and_upper 통과");

    println!("🎉 3.8 문자열 챕터 완료!");
}
