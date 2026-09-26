// SOLUTION - 5.4 스레드 (spawn/join, 채널을 통한 데이터 전송, Arc<Mutex>로 데이터 공유)
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;

// [문제 1] 스레드 생성 - 별도 스레드에서 계산하고 join으로 결과 받기
fn compute_in_thread(n: u32) -> u32 {
    let handle = thread::spawn(move || {
        (1..=n).sum::<u32>()
    });
    handle.join().unwrap()
}

// [문제 2] 채널(mpsc)로 여러 스레드 -> 메인 스레드로 데이터 전송
fn collect_from_threads(count: u32) -> Vec<u32> {
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];
    for i in 0..count {
        let tx = tx.clone();
        handles.push(thread::spawn(move || {
            tx.send(i * i).unwrap();
        }));
    }
    drop(tx); // 원본 sender도 닫아야 rx가 끝을 앎
    for h in handles {
        h.join().unwrap();
    }
    let mut results: Vec<u32> = rx.iter().collect();
    results.sort();
    results
}

// [문제 3] Arc<Mutex<T>>로 여러 스레드가 같은 카운터를 안전하게 공유하며 증가
fn parallel_counter(threads: u32, increments_each: u32) -> u32 {
    let counter = Arc::new(Mutex::new(0u32));
    let mut handles = vec![];
    for _ in 0..threads {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..increments_each {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    let result = *counter.lock().unwrap();
    result
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
