// https://www.acmicpc.net/problem/2798

use std::io::{self, Read};
use std::result::Result;
use std::str::FromStr;

fn read(s: String) -> Option<(i32, i32, Vec<i32>)> {
    let mut s = s.lines();

    let mut fst = s.next()?.split(' ');
    let n: i32 = fst.next()?.parse().unwrap();
    let m: i32 = fst.next()?.parse().unwrap();

    let snd: Vec<i32> = s
        .next()?
        .split(' ')
        .map(FromStr::from_str)
        .map(Result::unwrap)
        .collect();

    Some((n, m, snd))
}

fn find_max(m: i32, cards: Vec<i32>) -> i32 {
    let mut max = 0;

    for i in 0..cards.len() {
        for j in (i + 1)..cards.len() {
            for k in (j + 1)..cards.len() {
                let sum = cards[i] + cards[j] + cards[k];
                if sum <= m && sum > max {
                    max = sum;
                }
            }
        }
    }

    max
}

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();

    let (_, m, cards) = read(s).unwrap();

    let max = find_max(m, cards);

    println!("{}", max);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_example_1() {
        assert_eq!(
            read("5 21\n5 6 7 8 9".to_string()),
            Some((5, 21, vec![5, 6, 7, 8, 9]))
        )
    }

    #[test]
    fn read_example_2() {
        assert_eq!(
            read("10 500\n93 181 245 214 315 36 185 138 216 295".to_string()),
            Some((
                10,
                500,
                vec![93, 181, 245, 214, 315, 36, 185, 138, 216, 295]
            ))
        )
    }

    #[test]
    fn find_max_example_1() {
        assert_eq!(find_max(21, vec![5, 6, 7, 8, 9]), 21)
    }

    #[test]
    fn find_max_example_2() {
        assert_eq!(
            find_max(500, vec![93, 181, 245, 214, 315, 36, 185, 138, 216, 295]),
            497
        )
    }
}
