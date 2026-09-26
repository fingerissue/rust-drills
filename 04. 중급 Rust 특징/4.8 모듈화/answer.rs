// SOLUTION - 4.8 모듈화 (프로젝트/패키지/크레이트/모듈)
// 한 파일 안에서 mod 블록으로 모듈 구조를 연습한다.

// [문제 1] 모듈 정의 - pub으로 외부에서 접근 가능하게
mod shapes {
    pub struct Circle {
        pub radius: f64,
    }

    impl Circle {
        pub fn new(radius: f64) -> Self {
            Circle { radius }
        }

        pub fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }
    }

    // [문제 2] 하위 모듈 (nested module)
    pub mod utils {
        pub fn double(x: f64) -> f64 {
            x * 2.0
        }
    }
}

// [문제 3] 모듈 경로로 접근
fn make_circle_area(radius: f64) -> f64 {
    let c = shapes::Circle::new(radius);
    c.area()
}

// [문제 4] 하위 모듈 함수 사용
fn double_value(x: f64) -> f64 {
    shapes::utils::double(x)
}

// [문제 5] use로 경로 가져오기 + pub use로 재수출(re-export)
mod api {
    pub use super::shapes::Circle; // 재수출: api::Circle 로도 접근 가능
}
fn make_circle_via_api(radius: f64) -> f64 {
    use crate::api::Circle;
    Circle::new(radius).area()
}

// [문제 6] private 함수는 모듈 밖에서 호출 불가 - 모듈 내부에서만 쓰는 헬퍼
mod internal {
    fn secret_multiplier() -> f64 {
        3.0
    }
    pub fn triple(x: f64) -> f64 {
        x * secret_multiplier()
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
