/// 1로 만들기
/// https://www.acmicpc.net/problem/1463
/// 
/// # 배운 점
/// 벤치마크 테스트에서 컴파일러가 함수 콜을 컴파일 타임에 실행해서 상수로 만들어버리는 최적화를 방지하기 위해 test::black_box()
/// 함수를 쓸 수 있다는 걸 배웠다. [구현]을 살펴보니 LLVM은 인라인 어셈블리를 블랙박스 취급하기 때문에 값을 변경하지 않는 간단한
/// 인라인 어셈블리 콜을 하고 있는걸 확인할 수 있었다.
/// 
/// [구현]: https://doc.rust-lang.org/beta/src/core/hint.rs.html#109-119
// #![feature(test)]
// extern crate test;

use std::io;

fn min(n: i32) -> u8 {
    if n < 2 {
        0
    } else {
        (min(n / 2) + (n % 2) as u8).min(min(n / 3) + (n % 3) as u8) + 1
    }
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n: i32 = buf.trim().parse().unwrap();

    println!("{}", min(n));
}

#[cfg(test)]
mod tests {
    use super::*;
    // use test::Bencher;

    #[test]
    fn test() {
        assert_eq!(min(1), 0);
        assert_eq!(min(2), 1);
        assert_eq!(min(10), 3);
    }

    // #[bench]
    // fn bench_recursive(b: &mut Bencher) {
    //     b.iter(|| {
    //         let n = test::black_box(1_000_000);
    //         min(n)
    //     });
    // }
}
