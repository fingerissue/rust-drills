// Chapter 5.5 비동기 프로그래밍 (async/await, tokio)
// 실행: cargo run --bin problem  (이 폴더에서, cargo가 필요함 - tokio 의존성 때문)
use tokio::time::{sleep, Duration};

// [문제 1] 가장 단순한 async 함수 - "hello async" 반환
async fn hello() -> String {
    todo!()
}

// [문제 2] async 함수 안에서 다른 async 함수를 await해서 결과를 두 번 이어붙이기
async fn double_hello() -> String {
    todo!()
}

// [문제 3] tokio::spawn으로 여러 비동기 태스크를 동시에 실행하고 결과(제곱값) 모으기
async fn compute_all(values: Vec<u32>) -> Vec<u32> {
    let mut handles = vec![];
    todo!()
}

// [문제 4] tokio::fs로 비동기 파일 쓰기 후 다시 읽기
async fn write_and_read_async(path: &std::path::Path, content: &str) -> String {
    todo!()
}

#[tokio::main]
async fn main() {
    assert_eq!(hello().await, "hello async");
    println!("✅ 문제1 hello()(async 함수) 통과");

    assert_eq!(double_hello().await, "hello async hello async");
    println!("✅ 문제2 double_hello(await 체이닝) 통과");

    let results = compute_all(vec![1, 2, 3, 4]).await;
    assert_eq!(results, vec![1, 4, 9, 16]);
    println!("✅ 문제3 compute_all(tokio::spawn) 통과");

    let path = std::env::temp_dir().join(format!("async_drills_{}.txt", std::process::id()));
    let content = write_and_read_async(&path, "hello tokio fs").await;
    assert_eq!(content, "hello tokio fs");
    tokio::fs::remove_file(&path).await.ok();
    println!("✅ 문제4 write_and_read_async(tokio::fs) 통과");

    println!("🎉 5.5 비동기 프로그래밍 챕터 완료!");
}
