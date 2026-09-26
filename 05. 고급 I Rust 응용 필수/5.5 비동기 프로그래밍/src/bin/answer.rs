// SOLUTION - 5.5 비동기 프로그래밍 (async/await, tokio)
use tokio::time::{sleep, Duration};

// [문제 1] 가장 단순한 async 함수 - hello 반환
async fn hello() -> String {
    "hello async".to_string()
}

// [문제 2] async 함수 안에서 다른 async 함수를 await
async fn double_hello() -> String {
    let h = hello().await;
    format!("{} {}", h, h)
}

// [문제 3] tokio::spawn으로 여러 비동기 태스크를 동시에 실행하고 결과 모으기
async fn compute_all(values: Vec<u32>) -> Vec<u32> {
    let mut handles = vec![];
    for v in values {
        handles.push(tokio::spawn(async move {
            sleep(Duration::from_millis(10)).await; // 비동기 대기 시뮬레이션
            v * v
        }));
    }
    let mut results = vec![];
    for h in handles {
        results.push(h.await.unwrap());
    }
    results
}

// [문제 4] tokio::fs로 비동기 파일 쓰기/읽기
async fn write_and_read_async(path: &std::path::Path, content: &str) -> String {
    tokio::fs::write(path, content).await.unwrap();
    tokio::fs::read_to_string(path).await.unwrap()
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
