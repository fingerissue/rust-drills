// SOLUTION - 6.3 웹 서버, 웹 서비스 (Axum 0.6)
use axum::extract::Path;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;
use std::net::TcpListener as StdTcpListener;
use std::thread;
use std::time::Duration;

// [문제 1] Axum 핸들러 - "Hello Rust" 응답
async fn hello_handler() -> &'static str {
    "Hello Rust"
}

// [문제 2] Axum 핸들러 - 경로 파라미터(:name)를 받아서 인사말 생성
async fn greet_handler(Path(name): Path<String>) -> String {
    format!("안녕, {}!", name)
}

// [문제 3] Axum 핸들러 - JSON 응답
async fn status_handler() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}

// [문제 4] 라우터 구성 - 위 세 핸들러를 경로에 연결
fn build_router() -> Router {
    Router::new()
        .route("/", get(hello_handler))
        .route("/greet/:name", get(greet_handler))
        .route("/status", get(status_handler))
}

// 테스트 헬퍼 - 서버를 백그라운드 스레드에서 띄우고 접속 주소를 반환
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
