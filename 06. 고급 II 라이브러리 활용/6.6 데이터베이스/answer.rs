// SOLUTION - 6.6 데이터베이스 (SeaORM+MySQL 대신, 로컬에서 바로 테스트 가능한 SQLite로 같은
// 패턴(연결, Raw SQL, 파라미터 바인딩)을 연습한다. MySQL로 옮겨도 SQL/쿼리 패턴은 동일하다.)
use rusqlite::{params, Connection};

// [문제 1] 연결 + 테이블 생성 (Raw SQL)
fn setup_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute(
        "CREATE TABLE user (id INTEGER PRIMARY KEY, name TEXT NOT NULL, age INTEGER NOT NULL)",
        [],
    )
    .unwrap();
    conn
}

// [문제 2] 파라미터 바인딩으로 안전하게 INSERT (SQL 인젝션 방지)
fn insert_user(conn: &Connection, name: &str, age: i32) {
    conn.execute("INSERT INTO user (name, age) VALUES (?1, ?2)", params![name, age])
        .unwrap();
}

// [문제 3] Raw SQL로 조회 - 이름으로 나이 찾기
fn find_age_by_name(conn: &Connection, name: &str) -> Option<i32> {
    conn.query_row(
        "SELECT age FROM user WHERE name = ?1",
        params![name],
        |row| row.get(0),
    )
    .ok()
}

// [문제 4] 전체 개수 세기
fn count_users(conn: &Connection) -> i32 {
    conn.query_row("SELECT COUNT(*) FROM user", [], |row| row.get(0)).unwrap()
}

// [문제 5] 조건에 맞는 로우 삭제
fn delete_user(conn: &Connection, name: &str) {
    conn.execute("DELETE FROM user WHERE name = ?1", params![name]).unwrap();
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
