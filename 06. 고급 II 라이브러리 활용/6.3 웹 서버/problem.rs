// Chapter 6.3 웹 서버, 웹 서비스 (Axum)
// 실행: "06. 고급 II 라이브러리 활용" 폴더에서 cargo run --bin c63_problem
use axum::extract::Path;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;
use std::net::TcpListener as StdTcpListener;
use std::thread;
use std::time::Duration;

// [문제 1] Axum 핸들러 - "Hello Rust" 응답
async fn hello_handler() -> &'static str {
    todo!()
}

// [문제 2] Axum 핸들러 - 경로 파라미터(:name)를 받아서 "안녕, {name}!" 형태로 인사말 생성
async fn greet_handler(Path(name): Path<String>) -> String {
    todo!()
}

// [문제 3] Axum 핸들러 - {"status": "ok"} JSON 응답
async fn status_handler() -> Json<serde_json::Value> {
    todo!()
}

// [문제 4] 라우터 구성 - "/", "/greet/:name", "/status" 세 경로를 각 핸들러에 연결
fn build_router() -> Router {
    todo!()
}

// 테스트 헬퍼 (이미 완성되어 있음 - 손댈 필요 없음)
fn start_server() -> String {
    let std_listener = StdTcpListener::bind("127.0.0.1:0").unwrap();
    let addr = std_listener.local_addr().unwrap();
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            axum::Server::from_tcp(std_listener)
                .unwrap()
                .serve(build_router().into_make_service())
                .await
                .unwrap();
        });
    });
    format!("http://{}", addr)
}

fn main() {
    let base = start_server();
    thread::sleep(Duration::from_millis(300));

    let body = reqwest::blocking::get(&base).unwrap().text().unwrap();
    assert_eq!(body, "Hello Rust");
    println!("✅ 문제1 hello_handler 통과");

    let greet = reqwest::blocking::get(format!("{}/greet/치준환", base)).unwrap().text().unwrap();
    assert_eq!(greet, "안녕, 치준환!");
    println!("✅ 문제2 greet_handler(경로 파라미터) 통과");

    let json_resp: serde_json::Value =
        reqwest::blocking::get(format!("{}/status", base)).unwrap().json().unwrap();
    assert_eq!(json_resp["status"], "ok");
    println!("✅ 문제3 status_handler(JSON) 통과");

    println!("✅ 문제4 build_router(라우팅) 통과 (위 3개가 이미 라우터를 거쳐서 응답)");
    println!("🎉 6.3 웹 서버(Axum) 챕터 완료!");
}
