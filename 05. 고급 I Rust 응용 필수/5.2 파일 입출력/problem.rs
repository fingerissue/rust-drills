// Chapter 5.2 파일 입출력 (생성, 쓰기, 열기, 읽기, 디렉토리 관리)
// 실행: rustc problem.rs -o problem && ./problem
use std::fs;
use std::io::Write;
use std::path::Path;

// [문제 1] 파일 생성 + 쓰기
fn create_and_write(path: &Path, content: &str) -> std::io::Result<()> {
    todo!()
}

// [문제 2] 파일 읽기 - 내용을 문자열로 통째로 읽기
fn read_all(path: &Path) -> std::io::Result<String> {
    todo!()
}

// [문제 3] 파일에 이어쓰기 (append) - OpenOptions 사용
fn append_line(path: &Path, line: &str) -> std::io::Result<()> {
    use std::fs::OpenOptions;
    todo!()
}

// [문제 4] 파일 존재 여부 확인 (존재 + 파일인지)
fn file_exists(path: &Path) -> bool {
    todo!()
}

// [문제 5] 디렉토리 안의 파일 이름 목록을 정렬해서 반환
fn list_file_names(dir: &Path) -> std::io::Result<Vec<String>> {
    todo!()
}

// [문제 6] 파일 삭제
fn delete_file(path: &Path) -> std::io::Result<()> {
    todo!()
}

fn main() {
    let base = std::env::temp_dir().join(format!("rust_drills_test_{}", std::process::id()));
    fs::create_dir_all(&base).unwrap();

    let file_a = base.join("a.txt");
    create_and_write(&file_a, "hello").unwrap();
    assert_eq!(read_all(&file_a).unwrap(), "hello");
    println!("✅ 문제1,2 create_and_write/read_all 통과");

    append_line(&file_a, "world").unwrap();
    assert_eq!(read_all(&file_a).unwrap(), "helloworld\n");
    println!("✅ 문제3 append_line 통과");

    assert!(file_exists(&file_a));
    assert!(!file_exists(&base.join("없는파일.txt")));
    println!("✅ 문제4 file_exists 통과");

    create_and_write(&base.join("b.txt"), "b").unwrap();
    let names = list_file_names(&base).unwrap();
    assert_eq!(names, vec!["a.txt".to_string(), "b.txt".to_string()]);
    println!("✅ 문제5 list_file_names 통과");

    delete_file(&file_a).unwrap();
    assert!(!file_exists(&file_a));
    println!("✅ 문제6 delete_file 통과");

    fs::remove_dir_all(&base).ok();
    println!("🎉 5.2 파일 입출력 챕터 완료!");
}
