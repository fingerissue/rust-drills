// SOLUTION - 3.8 문자열 (&str, String)

// [문제 1] &str -> String 변환 + 접합
fn greet_name(name: &str) -> String {
    let mut s = String::from("안녕하세요, ");
    s.push_str(name);
    s.push('!');
    s
}

// [문제 2] 문자열 뒤집기 (유니코드 char 단위)
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

// [문제 3] 공백 기준 단어 개수 세기
fn word_count(s: &str) -> usize {
    s.split_whitespace().count()
}

// [문제 4] 특정 문자로 시작하는 단어만 추출
fn words_starting_with<'a>(s: &'a str, prefix: char) -> Vec<&'a str> {
    s.split_whitespace().filter(|w| w.starts_with(prefix)).collect()
}

// [문제 5] String 슬라이싱 - 앞 n바이트를 잘라서 반환 (영문 기준 문제)
fn truncate(s: &str, n: usize) -> String {
    s.chars().take(n).collect()
}

// [문제 6] 대소문자 변환 + trim
fn clean_and_upper(s: &str) -> String {
    s.trim().to_uppercase()
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
