// Chapter 4.8 모듈화 (프로젝트/패키지/크레이트/모듈)
// 실행: rustc problem.rs -o problem && ./problem
// 한 파일 안에서 mod 블록으로 모듈 구조를 연습한다.

mod shapes {
    pub struct Circle {
        pub radius: f64,
    }

    impl Circle {
        pub fn new(radius: f64) -> Self {
            Circle { radius }
        }

        // [문제 1] 원의 넓이 계산
        pub fn area(&self) -> f64 {
            todo!()
        }
    }

    // 하위 모듈 (nested module)
    pub mod utils {
        // [문제 2] 2배로 만드는 함수
        pub fn double(x: f64) -> f64 {
            todo!()
        }
    }
}

// [문제 3] 모듈 경로(shapes::Circle)로 접근해서 넓이 계산
fn make_circle_area(radius: f64) -> f64 {
    todo!()
}

// [문제 4] 하위 모듈 함수(shapes::utils::double) 사용
fn double_value(x: f64) -> f64 {
    todo!()
}

// pub use로 재수출(re-export) - api::Circle 로도 접근 가능하게 이미 되어있음
mod api {
    pub use super::shapes::Circle;
}
// [문제 5] use로 api::Circle을 가져와서 넓이 계산
fn make_circle_via_api(radius: f64) -> f64 {
    todo!()
}

mod internal {
    fn secret_multiplier() -> f64 {
        3.0
    }
    // [문제 6] private 헬퍼(secret_multiplier)를 이용해 3배로 만드는 public 함수
    pub fn triple(x: f64) -> f64 {
        todo!()
    }
}
fn triple_value(x: f64) -> f64 {
    internal::triple(x)
}

fn main() {
    assert!((make_circle_area(2.0) - 12.566370614).abs() < 1e-6);
    println!("✅ 문제1,3 shapes::Circle(모듈+pub) 통과");

    assert_eq!(double_value(5.0), 10.0);
    println!("✅ 문제2,4 shapes::utils(하위 모듈) 통과");

    assert!((make_circle_via_api(1.0) - std::f64::consts::PI).abs() < 1e-6);
    println!("✅ 문제5 api::Circle(재수출) 통과");

    assert_eq!(triple_value(4.0), 12.0);
    println!("✅ 문제6 internal::triple(private 헬퍼) 통과");

    println!("🎉 4.8 모듈화 챕터 완료!");
}
