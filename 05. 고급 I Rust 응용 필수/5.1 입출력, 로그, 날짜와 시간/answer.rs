// SOLUTION - 5.1 입출력, 로그, 날짜와 시간
use std::time::Duration;

// [문제 1] 커맨드 라인 입력 - 실제로는 std::env::args()로 받지만,
// 테스트하기 쉽게 인자 벡터를 직접 받는 형태로 연습한다.
// 첫 번째 인자를 프로그램 이름으로 보고, 그 뒤 인자들만 반환.
fn parse_args(args: &[String]) -> Vec<String> {
    args[1..].to_vec()
}

// [문제 2] 콘솔 입력 파싱 - stdin().read_line()으로 받은 한 줄을
// trim + parse 해서 숫자로 바꾸는 패턴 (여기선 함수 인자로 시뮬레이션)
fn parse_input_number(line: &str) -> Option<i32> {
    line.trim().parse::<i32>().ok()
}

// [문제 3] 로그 출력 - 로그 레벨에 따라 다른 접두어를 붙이는 간단한 로거
#[derive(Debug, PartialEq)]
enum LogLevel {
    Info,
    Warn,
    Error,
}
fn format_log(level: LogLevel, msg: &str) -> String {
    let prefix = match level {
        LogLevel::Info => "[INFO]",
        LogLevel::Warn => "[WARN]",
        LogLevel::Error => "[ERROR]",
    };
    format!("{} {}", prefix, msg)
}

// [문제 4] 시간 - Duration을 이용해서 초 단위 값을 "HH:MM:SS"로 변환
fn format_duration(d: Duration) -> String {
    let total_secs = d.as_secs();
    let h = total_secs / 3600;
    let m = (total_secs % 3600) / 60;
    let s = total_secs % 60;
    format!("{:02}:{:02}:{:02}", h, m, s)
}

// [문제 5] 시간 - 두 Duration을 더한 뒤 다시 "HH:MM:SS"로 변환
fn add_durations(a: Duration, b: Duration) -> String {
    format_duration(a + b)
}

fn main() {
    let args = vec!["program".to_string(), "hello".to_string(), "world".to_string()];
    assert_eq!(parse_args(&args), vec!["hello", "world"]);
    println!("✅ 문제1 parse_args(커맨드 라인) 통과");

    assert_eq!(parse_input_number("  42\n"), Some(42));
    assert_eq!(parse_input_number("abc"), None);
    println!("✅ 문제2 parse_input_number(콘솔 입력) 통과");

    assert_eq!(format_log(LogLevel::Info, "서버 시작"), "[INFO] 서버 시작");
    assert_eq!(format_log(LogLevel::Error, "연결 실패"), "[ERROR] 연결 실패");
    println!("✅ 문제3 format_log 통과");

    assert_eq!(format_duration(Duration::from_secs(3661)), "01:01:01");
    println!("✅ 문제4 format_duration 통과");

    assert_eq!(add_durations(Duration::from_secs(1800), Duration::from_secs(1800)), "01:00:00");
    println!("✅ 문제5 add_durations 통과");

    println!("🎉 5.1 입출력/로그/날짜시간 챕터 완료!");
}
