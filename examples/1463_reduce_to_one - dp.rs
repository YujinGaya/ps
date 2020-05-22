#![feature(test)]
extern crate test;

use std::io;

fn min_steps(n: usize) -> Vec<u8> {
    let mut v = vec![0; n + 1];
    for i in 2..=n {
        v[i] = v[i - 1]
            .min(v[i / 2] + (i % 2) as u8)
            .min(v[i / 3] + (i % 3) as u8)
            + 1;
    }

    v
}

fn min_step(n: usize) -> u8 {
    min_steps(n)[n]
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    println!("{}", min_step(n));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test() {
        assert_eq!(min_step(1), 0);
        assert_eq!(min_step(2), 1);
        assert_eq!(min_step(10), 3);
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| min_steps(1_000_000));
    }
}
