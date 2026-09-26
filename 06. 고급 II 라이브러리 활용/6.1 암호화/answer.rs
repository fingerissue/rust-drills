// SOLUTION - 6.1 암호화 (난수 생성, 해시, 대칭키 암호)
use rand::Rng;
use sha2::{Digest, Sha256};
use aes_gcm::aead::{Aead, KeyInit, OsRng as AesOsRng};
use aes_gcm::{Aes256Gcm, Nonce};
use aes_gcm::aead::rand_core::RngCore;

// [문제 1] 난수 생성 - 지정된 범위 안의 난수 n개를 생성
fn generate_random_numbers(count: usize, min: i32, max: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..count).map(|_| rng.gen_range(min..=max)).collect()
}

// [문제 2] 해시 - SHA-256으로 문자열을 해싱해서 16진수 문자열로 반환
fn sha256_hex(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    result.iter().map(|b| format!("{:02x}", b)).collect()
}

// [문제 3] 대칭키 암호 - AES-256-GCM으로 암호화/복호화 왕복
fn encrypt_decrypt_roundtrip(plaintext: &str) -> String {
    let key = Aes256Gcm::generate_key(&mut AesOsRng);
    let cipher = Aes256Gcm::new(&key);

    let mut nonce_bytes = [0u8; 12];
    AesOsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, plaintext.as_bytes()).unwrap();
    let decrypted = cipher.decrypt(nonce, ciphertext.as_ref()).unwrap();
    String::from_utf8(decrypted).unwrap()
}

fn main() {
    let nums = generate_random_numbers(20, 1, 10);
    assert_eq!(nums.len(), 20);
    assert!(nums.iter().all(|&n| n >= 1 && n <= 10));
    println!("✅ 문제1 generate_random_numbers 통과");

    let h1 = sha256_hex("hello");
    let h2 = sha256_hex("hello");
    let h3 = sha256_hex("world");
    assert_eq!(h1, h2); // 같은 입력 -> 같은 해시
    assert_ne!(h1, h3); // 다른 입력 -> 다른 해시
    assert_eq!(h1.len(), 64); // SHA-256 = 32바이트 = 16진수 64자
    println!("✅ 문제2 sha256_hex 통과");

    assert_eq!(encrypt_decrypt_roundtrip("치준환의 비밀 메시지"), "치준환의 비밀 메시지");
    println!("✅ 문제3 encrypt_decrypt_roundtrip(AES-256-GCM) 통과");

    println!("🎉 6.1 암호화 챕터 완료!");
}
