// Chapter 5.4 스레드 (spawn/join, mpsc 채널, Arc<Mutex>로 데이터 공유)
// 실행: rustc problem.rs -o problem && ./problem
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;

// [문제 1] 스레드 생성 - 별도 스레드에서 1부터 n까지 합을 계산하고 join으로 결과 받기
fn compute_in_thread(n: u32) -> u32 {
    todo!()
}

// [문제 2] 채널(mpsc)로 여러 스레드가 계산한 값(i*i)을 모아서 정렬해 반환
fn collect_from_threads(count: u32) -> Vec<u32> {
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];
    todo!()
}

// [문제 3] Arc<Mutex<T>>로 여러 스레드가 같은 카운터를 안전하게 공유하며 증가
fn parallel_counter(threads: u32, increments_each: u32) -> u32 {
    let counter = Arc::new(Mutex::new(0u32));
    let mut handles = vec![];
    todo!()
}

fn main() {
    assert_eq!(compute_in_thread(10), 55);
    println!("✅ 문제1 compute_in_thread(spawn/join) 통과");

    assert_eq!(collect_from_threads(5), vec![0, 1, 4, 9, 16]);
    println!("✅ 문제2 collect_from_threads(mpsc 채널) 통과");

    assert_eq!(parallel_counter(4, 1000), 4000);
    println!("✅ 문제3 parallel_counter(Arc<Mutex>) 통과");

    println!("🎉 5.4 스레드 챕터 완료!");
}
