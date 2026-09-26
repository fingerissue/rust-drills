// Chapter 5.3 네트워크 프로그래밍 (TCP 서버/클라이언트, UDP)
// 실행: rustc problem.rs -o problem && ./problem
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, UdpSocket};
use std::thread;

// [문제 1] TCP 서버 - 클라이언트로부터 한 번 받아서 그대로 돌려주는(echo) 서버
fn run_echo_server(listener: TcpListener) {
    todo!()
}

// [문제 2] TCP 클라이언트 - 서버에 메시지를 보내고 응답을 받아 문자열로 반환
fn send_and_receive(addr: &str, msg: &str) -> String {
    todo!()
}

// [문제 3] UDP - 보내기/받기
fn udp_send(socket: &UdpSocket, target: &str, msg: &str) {
    todo!()
}
fn udp_recv(socket: &UdpSocket) -> String {
    todo!()
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap().to_string();
    let handle = thread::spawn(move || run_echo_server(listener));
    thread::sleep(std::time::Duration::from_millis(100));

    let response = send_and_receive(&addr, "hello tcp");
    assert_eq!(response, "hello tcp");
    handle.join().unwrap();
    println!("✅ 문제1,2 TCP echo 서버/클라이언트 통과");

    let server_socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let server_addr = server_socket.local_addr().unwrap().to_string();
    let client_socket = UdpSocket::bind("127.0.0.1:0").unwrap();

    udp_send(&client_socket, &server_addr, "hello udp");
    let received = udp_recv(&server_socket);
    assert_eq!(received, "hello udp");
    println!("✅ 문제3 UDP send/recv 통과");

    println!("🎉 5.3 네트워크 프로그래밍(TCP/UDP) 챕터 완료!");
}
