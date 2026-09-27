# 🦀 rust-drills — `seed` 브랜치

> ⚠️ **이 브랜치엔 커밋하지 않는다.** 문제은행이 막 완성된 시점(전부 `todo!()`, 아무것도 안 풀림)의
> 스냅샷을 그대로 얼려둔 보관용 브랜치다.
>
> 실제로 문제를 풀고 커밋하는 곳은 **[`grass`](https://github.com/fingerissue/rust-drills/tree/grass) 브랜치**
> (기본 브랜치)다. 웹페이지, Actions 자동 채점, 진행 상황도 전부 거기 있음.

## 🔗 바로가기

| | |
|---|---|
| 🌱 실제 풀이/커밋하는 브랜치 (`grass`) | https://github.com/fingerissue/rust-drills/tree/grass |
| 🌐 웹에서 풀기 | https://fingerissue.github.io/rust-drills/ |

## 이 브랜치는 언제 쓰나

- "이 문제 원래 모양이 뭐였지?" 확인하고 싶을 때
- 어떤 챕터를 처음부터 완전히 다시 풀고 싶을 때 (여기서 해당 폴더만 복사해가면 됨)

---

위키독스 [Just Do Rust - 러스트 기초부터 고급까지](https://wikidocs.net/book/16747)를 읽으면서
챕터별로 복습 문제를 풀어보기 위한 개인 학습용 레포.

## 폴더 구조

책의 목차를 그대로 따라간다. 너무 잘게 쪼개진 하위 목차(예: 3.6.1~3.6.5)는 상위 목차 하나로 합쳤다.

```
01. 프로그래밍 환경 구축하기/         (1.1 ~ 1.3)
02. 입문/                          (2.1 ~ 2.2)
03. 초급 Rust 기본 문법/            (3.1 ~ 3.9, 9개 섹션)
04. 중급 Rust 특징/                 (4.1 ~ 4.9, 9개 섹션)
05. 고급 I Rust 응용 필수/           (5.1 ~ 5.6, 5.5만 별도 Cargo 프로젝트)
06. 고급 II 라이브러리 활용/         (6.1~6.3, 6.6은 코드 / 6.4, 6.5는 개념정리.md)
07. 부록/                          (책엔 없지만 필수급 개념 4개 - 7.1~7.4, 전부 개념설명.md 포함)
```

`07. 부록`은 책 목차엔 없지만 실무에서 자주 쓰여서 추가한 챕터다: Rc/RefCell/Weak, 테스트(`#[test]`),
타입 변환(From/Into/TryFrom), 클로저 심화(Fn/FnMut/FnOnce). 여기만 `problem.rs`/`answer.rs` 외에
`개념설명.md`가 항상 같이 들어있다.

각 하위 폴더는 보통 `problem.rs`(TODO를 채우는 문제) + `answer.rs`(정답) 짝으로 구성되고,
코드가 없는 챕터나 개념 위주 챕터는 `개념정리.md` / `체크리스트.md`로 대체했다.
모든 문제는 책(PDF) 없이 `problem.rs` 안의 설명만 보고 풀 수 있게 자기완결적으로 작성함.

## 실행법 (참고용)

```bash
rustc problem.rs -o problem && ./problem
```
`5.5`, `06장`은 외부 크레이트가 필요해서 각각 독립 Cargo 프로젝트다 — 자세한 건
[`grass`의 README](https://github.com/fingerissue/rust-drills/blob/grass/README.md) 참고.

## 이 스냅샷 시점 기준 완성 범위

- [x] 01. 프로그래밍 환경 구축하기
- [x] 02. 입문
- [x] 03. 초급: Rust 기본 문법
- [x] 04. 중급: Rust 특징
- [x] 05. 고급 I: Rust 응용 필수
- [x] 06. 고급 II: 라이브러리 활용 (문제은행 기준 완료 — 책 목차 전체 커버)
- [x] 07. 부록: Rc/RefCell/Weak, 테스트, From/Into, 클로저 심화

실제 풀이 진행 상황은 [`grass`](https://github.com/fingerissue/rust-drills/tree/grass) 브랜치에서 확인.

---

기획/구조 설계: [@fingerissue](https://github.com/fingerissue) · 문제 작성: Claude(Anthropic)가 책 목차와 본문을 참고해서 만듦.
