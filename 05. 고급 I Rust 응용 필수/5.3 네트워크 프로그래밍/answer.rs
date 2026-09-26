// SOLUTION - 5.3 네트워크 프로그래밍 (TCP 서버/클라이언트, UDP)
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, UdpSocket};
use std::thread;

// [문제 1] TCP 서버 - 클라이언트로부터 한 줄 받아서 그대로 돌려주는(echo) 서버
fn run_echo_server(listener: TcpListener) {
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buf = [0u8; 512];
        let n = stream.read(&mut buf).unwrap();
        stream.write_all(&buf[..n]).unwrap();
        break; // 테스트용: 한 번만 처리하고 종료
    }
}

// [문제 2] TCP 클라이언트 - 서버에 메시지를 보내고 응답을 받기
fn send_and_receive(addr: &str, msg: &str) -> String {
    let mut stream = TcpStream::connect(addr).unwrap();
    stream.write_all(msg.as_bytes()).unwrap();
    let mut buf = [0u8; 512];
    let n = stream.read(&mut buf).unwrap();
    String::from_utf8_lossy(&buf[..n]).to_string()
}

// [문제 3] UDP - 소켓 하나로 메시지를 보내고, 다른 소켓으로 받기
fn udp_send(socket: &UdpSocket, target: &str, msg: &str) {
    socket.send_to(msg.as_bytes(), target).unwrap();
}
fn udp_recv(socket: &UdpSocket) -> String {
    let mut buf = [0u8; 512];
    let (n, _addr) = socket.recv_from(&mut buf).unwrap();
    String::from_utf8_lossy(&buf[..n]).to_string()
}

fn main() {
    // TCP echo 테스트
    let listener = TcpListener::bind("127.0.0.1:0").unwrap(); // 포트 0 = OS가 빈 포트 자동 할당
    let addr = listener.local_addr().unwrap().to_string();
    let handle = thread::spawn(move || run_echo_server(listener));
    thread::sleep(std::time::Duration::from_millis(100));

    let response = send_and_receive(&addr, "hello tcp");
    assert_eq!(response, "hello tcp");
    handle.join().unwrap();
    println!("✅ 문제1,2 TCP echo 서버/클라이언트 통과");

    // UDP 테스트
    let server_socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let server_addr = server_socket.local_addr().unwrap().to_string();
    let client_socket = UdpSocket::bind("127.0.0.1:0").unwrap();

    udp_send(&client_socket, &server_addr, "hello udp");
    let received = udp_recv(&server_socket);
    assert_eq!(received, "hello udp");
    println!("✅ 문제3 UDP send/recv 통과");

    println!("🎉 5.3 네트워크 프로그래밍(TCP/UDP) 챕터 완료!");
}
