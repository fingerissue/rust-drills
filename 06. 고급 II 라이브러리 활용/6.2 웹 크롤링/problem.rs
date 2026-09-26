// Chapter 6.2 웹 크롤링 (reqwest, scraper)
// 실행: "06. 고급 II 라이브러리 활용" 폴더에서 cargo run --bin c62_problem
// 실제 인터넷 대신, 로컬 TCP 서버를 하나 띄워서 크롤링 대상으로 삼는다.
use scraper::{Html, Selector};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

// [문제 1] scraper - HTML에서 모든 <li> 텍스트를 추출
fn extract_list_items(html: &str) -> Vec<String> {
    let doc = Html::parse_document(html);
    let selector = Selector::parse("li").unwrap();
    todo!()
}

// [문제 2] scraper - 모든 <a> 태그의 href 속성을 추출
fn extract_links(html: &str) -> Vec<String> {
    let doc = Html::parse_document(html);
    let selector = Selector::parse("a").unwrap();
    todo!()
}

// 테스트용 미니 HTTP 서버 (이미 완성되어 있음 - 손댈 필요 없음)
fn start_mock_server() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    thread::spawn(move || {
        if let Ok((mut stream, _)) = listener.accept() {
            let mut buf = [0u8; 1024];
            let _ = stream.read(&mut buf);
            let body = "<html><body><ul><li>Rust</li><li>Python</li></ul></body></html>";
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
                body.len(),
                body
            );
            let _ = stream.write_all(response.as_bytes());
        }
    });
    format!("http://{}", addr)
}

// [문제 3] reqwest(blocking) - GET 요청 보내고 본문을 문자열로 받기
fn fetch_body(url: &str) -> String {
    todo!()
}

// [문제 4] reqwest + scraper 조합 - 가져온 HTML에서 리스트 아이템 추출 (문제1, 3 함수 재사용)
fn fetch_and_extract_list(url: &str) -> Vec<String> {
    todo!()
}

fn main() {
    let sample_html = r#"
        <html><body>
            <ul>
                <li>사과</li>
                <li>바나나</li>
            </ul>
            <a href="https://a.com">A</a>
            <a href="https://b.com">B</a>
        </body></html>
    "#;

    assert_eq!(extract_list_items(sample_html), vec!["사과", "바나나"]);
    println!("✅ 문제1 extract_list_items(scraper) 통과");

    assert_eq!(extract_links(sample_html), vec!["https://a.com", "https://b.com"]);
    println!("✅ 문제2 extract_links(scraper) 통과");

    let url = start_mock_server();
    thread::sleep(std::time::Duration::from_millis(100));
    let body = fetch_body(&url);
    assert!(body.contains("Rust"));
    println!("✅ 문제3 fetch_body(reqwest) 통과");

    let url2 = start_mock_server();
    thread::sleep(std::time::Duration::from_millis(100));
    assert_eq!(fetch_and_extract_list(&url2), vec!["Rust", "Python"]);
    println!("✅ 문제4 fetch_and_extract_list(reqwest+scraper) 통과");

    println!("🎉 6.2 웹 크롤링 챕터 완료!");
}
