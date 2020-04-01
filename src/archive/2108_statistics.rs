/// 통계학
/// https://acmicpc.net/problem/2108
/// 
/// Divide toward zero나 실수하기 쉬운 엣지 케이스 구현 연습하기 좋은 문제인듯.
use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let mut v: Vec<i32> = io::stdin()
        .lock()
        .lines()
        .skip(1)
        .map(|n| n.unwrap().parse().unwrap())
        .collect();

    v.sort();

    let mut m = HashMap::new();

    for i in v.iter() {
        *m.entry(i).or_insert(0) += 1;
    }

    let mut most_freq = 0;
    let mut mode_smallest = i32::max_value();
    let mut mode_second_smallest = i32::max_value();

    for (&&i, &n) in m.iter() {
        if most_freq <= n {
            if most_freq < n {
                mode_second_smallest = i32::max_value();
                mode_smallest = i;
            } else {
                if mode_smallest > i {
                    mode_second_smallest = mode_smallest;
                    mode_smallest = i;
                } else if mode_second_smallest > i {
                    mode_second_smallest = i;
                }
            }
            most_freq = n;
        }
    }

    let sum = v.iter().sum::<i32>();
    let len = v.len() as i32;

    println!(
        "{}",
        sum / len
            + if sum > 0 {
                if sum % len * 2 > len {
                    1
                } else {
                    0
                }
            } else {
                if sum % len * -2 > len {
                    -1
                } else {
                    0
                }
            }
    );
    println!("{}", v[v.len() / 2]);
    println!(
        "{}",
        if mode_second_smallest == i32::max_value() {
            mode_smallest
        } else {
            mode_second_smallest
        }
    );
    println!("{}", v.iter().max().unwrap() - v.iter().min().unwrap());
}
