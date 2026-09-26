// Chapter 6.1 암호화 (난수 생성, 해시, 대칭키 암호)
// 실행: 이 폴더가 아니라 "06. 고급 II 라이브러리 활용" 폴더에서
//      cargo run --bin c61_problem
use rand::Rng;
use sha2::{Digest, Sha256};
use aes_gcm::aead::{Aead, KeyInit, OsRng as AesOsRng};
use aes_gcm::{Aes256Gcm, Nonce};
use aes_gcm::aead::rand_core::RngCore;

// [문제 1] 난수 생성 - 지정된 범위(min..=max) 안의 난수 n개를 생성
fn generate_random_numbers(count: usize, min: i32, max: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    todo!()
}

// [문제 2] 해시 - SHA-256으로 문자열을 해싱해서 16진수 문자열로 반환
fn sha256_hex(input: &str) -> String {
    let mut hasher = Sha256::new();
    todo!()
}

// [문제 3] 대칭키 암호 - AES-256-GCM으로 암호화/복호화 왕복 (원문 그대로 복원)
fn encrypt_decrypt_roundtrip(plaintext: &str) -> String {
    let key = Aes256Gcm::generate_key(&mut AesOsRng);
    let cipher = Aes256Gcm::new(&key);

    // TODO: 12바이트 난수 nonce를 만들고(AesOsRng.fill_bytes),
    // cipher.encrypt(nonce, plaintext.as_bytes())로 암호화한 뒤
    // cipher.decrypt(nonce, ciphertext)로 복호화해서 원문 문자열로 돌려줘라.
    todo!()
}

fn main() {
    let nums = generate_random_numbers(20, 1, 10);
    assert_eq!(nums.len(), 20);
    assert!(nums.iter().all(|&n| n >= 1 && n <= 10));
    println!("✅ 문제1 generate_random_numbers 통과");

    let h1 = sha256_hex("hello");
    let h2 = sha256_hex("hello");
    let h3 = sha256_hex("world");
    assert_eq!(h1, h2);
    assert_ne!(h1, h3);
    assert_eq!(h1.len(), 64);
    println!("✅ 문제2 sha256_hex 통과");

    assert_eq!(encrypt_decrypt_roundtrip("치준환의 비밀 메시지"), "치준환의 비밀 메시지");
    println!("✅ 문제3 encrypt_decrypt_roundtrip(AES-256-GCM) 통과");

    println!("🎉 6.1 암호화 챕터 완료!");
}
