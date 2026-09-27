# 🦀 rust-drills

위키독스 [Just Do Rust - 러스트 기초부터 고급까지](https://wikidocs.net/book/16747)를 읽으면서
챕터별로 복습 문제를 직접 풀어보기 위한 개인 학습용 레포.

**여기(`grass`)가 실제로 문제를 풀고 커밋하는 브랜치다.** 커밋 하나하나가 "그날 이만큼 풀었다"는 학습 기록이자 잔디.

## 🔗 바로가기

| | |
|---|---|
| 🌐 웹에서 풀기 (모바일 포함) | https://fingerissue.github.io/rust-drills/ |
| 📦 원본 문제은행 (`seed`, 전부 미해결 상태) | https://github.com/fingerissue/rust-drills/tree/seed |
| ⚙️ 채점 현황 (Actions) | https://github.com/fingerissue/rust-drills/actions |

[![Grade Submission](https://github.com/fingerissue/rust-drills/actions/workflows/grade.yml/badge.svg?branch=grass)](https://github.com/fingerissue/rust-drills/actions/workflows/grade.yml)

## 진행 상황

- [x] 01. 프로그래밍 환경 구축하기
- [x] 02. 입문
- [x] 03. 초급: Rust 기본 문법
- [x] 04. 중급: Rust 특징
- [x] 05. 고급 I: Rust 응용 필수
- [x] 06. 고급 II: 라이브러리 활용

문제은행(전체 목차)은 이미 완성된 상태고, 위 체크는 문제은행 기준이다.
**실제로 "오늘 몇 챕터까지 풀었다"는 진도는 커밋 히스토리로 확인**하는 게 정확하다
(문제은행에 올라가 있다고 그날 다 공부한 건 아님 — 업로드일 ≠ 학습일).

## 폴더 구조

책의 목차를 그대로 따라간다. 너무 잘게 쪼개진 하위 목차(예: 3.6.1~3.6.5)는 상위 목차 하나로 합쳤다.

```
01. 프로그래밍 환경 구축하기/
  1.1 Rust 설치하기/            (체크리스트.md)
  1.2 VS Code 설치하기/          (체크리스트.md)
  1.3 첫 번째 프로그래밍 - Hello world/  (problem.rs, answer.rs)
02. 입문/
  2.1 무작정 따라하며 Rust 코드 짜 보기/
  2.2 작성한 코드 이해하기/
03. 초급 Rust 기본 문법/         (3.1 ~ 3.9, 9개 섹션)
04. 중급 Rust 특징/              (4.1 ~ 4.9, 9개 섹션)
05. 고급 I Rust 응용 필수/        (5.1 ~ 5.6, 5.5만 별도 Cargo 프로젝트)
06. 고급 II 라이브러리 활용/      (6.1~6.3, 6.6은 코드 / 6.4, 6.5는 개념정리.md)
```

각 하위 폴더는 보통 `problem.rs`(TODO를 채우는 문제) + `answer.rs`(정답) 짝으로 구성되고,
코드가 없는 챕터(1.1, 1.2)나 개념 위주 챕터(3.4, 6.4, 6.5)는 `개념정리.md` / `체크리스트.md`로 대체했다.
모든 문제는 책(PDF) 없이 `problem.rs` 안의 설명만 보고 풀 수 있게 자기완결적으로 작성함.

## 사용법

### 로컬
대부분의 폴더에서:
```bash
rustc problem.rs -o problem && ./problem
```

예외:
- **`5.5 비동기 프로그래밍`**: tokio 필요, 그 폴더 자체가 독립 Cargo 프로젝트
  ```bash
  cd "05. 고급 I Rust 응용 필수/5.5 비동기 프로그래밍"
  cargo run --bin problem
  ```
- **`06. 고급 II 라이브러리 활용`**: 챕터 전체(6.1, 6.2, 6.3, 6.6)가 외부 크레이트(rand, sha2, aes-gcm,
  reqwest, scraper, axum, tokio, rusqlite) 필요, 챕터 폴더 하나가 통째로 Cargo 프로젝트
  ```bash
  cd "06. 고급 II 라이브러리 활용"
  cargo run --bin c61_problem   # 6.1 암호화
  cargo run --bin c62_problem   # 6.2 웹 크롤링
  cargo run --bin c63_problem   # 6.3 웹 서버
  cargo run --bin c66_problem   # 6.6 데이터베이스 (SeaORM+MySQL 대신 SQLite로 같은 패턴 연습)
  ```

### 웹 (모바일 포함)
https://fingerissue.github.io/rust-drills/ 에서 챕터별 문제를 보고, 에디터에서 TODO를 채운 뒤
"Rust Playground에서 실행" 버튼으로 컴파일/실행 결과를 확인할 수 있다.
(정적 호스팅이라 브라우저 안에서 직접 컴파일은 불가 — 탭 한 번으로 Playground로 코드를 넘기는 방식.
`5.5`, `6.1~6.3`, `6.6`처럼 외부 크레이트가 필요한 문제는 Playground에서 안 돌아갈 수 있어서 로컬 cargo 권장)

`docs/data.js`는 각 폴더의 `problem.rs`/`answer.rs`/`.md`를 모아놓은 파일이라,
새 챕터 추가 시 `python3 scripts/gen_data.py`로 재생성해야 한다.

## 자동 채점 (GitHub Actions)

`problem.rs`를 고쳐서 push하면 `.github/workflows/grade.yml`이 트리거돼서:
1. 변경된 `problem.rs` 파일들을 찾고
2. 표준 폴더는 `rustc`로 컴파일 후 실행, Cargo 프로젝트(5.5, 06장)는 해당 `--bin`으로 `cargo run`
3. `main()` 안의 `assert_eq!`가 전부 통과하면 ✅, 하나라도 실패(panic)하면 ❌

결과는 커밋의 체크(check)로 남아서 커밋 목록/PR 화면에서 바로 보이고, [Actions 탭](https://github.com/fingerissue/rust-drills/actions)에서 로그도 확인 가능.

---

기획/구조 설계: [@fingerissue](https://github.com/fingerissue) · 문제 작성: Claude(Anthropic)가 책 목차와 본문을 참고해서 만듦.
