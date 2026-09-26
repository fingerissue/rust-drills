// Chapter 6.6 데이터베이스
// (SeaORM+MySQL 대신, 로컬에서 바로 실행 가능한 SQLite로 같은 패턴을 연습한다.
//  연결/Raw SQL/파라미터 바인딩 패턴은 MySQL로 옮겨도 거의 동일하다.)
// 실행: "06. 고급 II 라이브러리 활용" 폴더에서 cargo run --bin c66_problem
use rusqlite::{params, Connection};

// [문제 1] 연결 + 테이블 생성 (Raw SQL)
fn setup_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    todo!()
}

// [문제 2] 파라미터 바인딩으로 안전하게 INSERT (SQL 인젝션 방지)
fn insert_user(conn: &Connection, name: &str, age: i32) {
    todo!()
}

// [문제 3] Raw SQL로 조회 - 이름으로 나이 찾기 (없으면 None)
fn find_age_by_name(conn: &Connection, name: &str) -> Option<i32> {
    todo!()
}

// [문제 4] 전체 개수 세기
fn count_users(conn: &Connection) -> i32 {
    todo!()
}

// [문제 5] 조건에 맞는 로우 삭제
fn delete_user(conn: &Connection, name: &str) {
    todo!()
}

fn main() {
    let conn = setup_db();
    println!("✅ 문제1 setup_db(연결+테이블 생성) 통과");

    insert_user(&conn, "치준환", 20);
    insert_user(&conn, "동료", 25);
    println!("✅ 문제2 insert_user(파라미터 바인딩) 통과");

    assert_eq!(find_age_by_name(&conn, "치준환"), Some(20));
    assert_eq!(find_age_by_name(&conn, "없는사람"), None);
    println!("✅ 문제3 find_age_by_name(Raw SQL 조회) 통과");

    assert_eq!(count_users(&conn), 2);
    println!("✅ 문제4 count_users 통과");

    delete_user(&conn, "동료");
    assert_eq!(count_users(&conn), 1);
    println!("✅ 문제5 delete_user 통과");

    println!("🎉 6.6 데이터베이스 챕터 완료!");
}
