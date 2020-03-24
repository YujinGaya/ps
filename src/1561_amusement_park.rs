// 1561_amusement_park

use std::io::{self, Read};
use std::result::Result;
use std::str::FromStr;

const MAX_TICKS: i64 = 2_000_000_000 * 30;

fn parse(s: &str) -> (i64, i32, Vec<i8>) {
    let mut s = s.lines();

    let mut fst = s.next().unwrap().split(' ');
    let n = fst.next().unwrap().parse().unwrap();
    let m = fst.next().unwrap().parse().unwrap();

    let snd = s
        .next()
        .unwrap()
        .split(' ')
        .map(FromStr::from_str)
        .map(Result::unwrap)
        .collect();

    (n, m, snd)
}

/// returns how many children rode the rides
fn tick_by(minutes: i64, ride_durations: &Vec<i8>) -> i64 {
    ride_durations
        .iter()
        .map(|&d| (minutes / (d as i64) + if minutes % (d as i64) == 0 { 0 } else { 1 }))
        .sum::<i64>()
}

/// f takes time and returns number of children rode the rides
fn bsearch<F>(start: i64, end: i64, f: F, children: i64) -> i64
where
    F: Fn(i64) -> i64,
{
    let middle = start + (end - start) / 2;

    if start == end || start == middle {
        return start;
    }

    if children <= f(middle) {
        bsearch(start, middle, f, children)
    } else {
        bsearch(middle, end, f, children)
    }
}

fn empty_rides_after<'a>(
    ticks: i64,
    rides_durations: &'a Vec<i8>,
) -> impl Iterator<Item = bool> + 'a {
    rides_durations
        .iter()
        .map(move |&d| ticks % (d as i64) == 0)
}

fn last_ride(children: i64, ticks: i64, rides_durations: &Vec<i8>) -> usize {
    empty_rides_after(ticks, rides_durations)
        .enumerate()
        .filter(|&(_, d)| d)
        .nth((children - tick_by(ticks, &rides_durations) - 1) as usize)
        .unwrap()
        .0
        + 1
}

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();

    let (children, _, durations) = parse(&s);

    let ticks = bsearch(0, MAX_TICKS, |t| tick_by(t, &durations), children);

    let ride = last_ride(children, ticks, &durations);

    println!("{}", ride);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_normal() {
        assert_eq!(parse("22 5\n1 2 3 4 5\n"), (22, 5, vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn tick_by_normal() {
        assert_eq!(tick_by(60, &vec![1, 2, 3, 4, 5]), 60 + 30 + 20 + 15 + 12);
    }

    #[test]
    fn tick_by_11() {
        assert_eq!(tick_by(11, &vec![1, 2, 3, 4, 5]), 11 + 6 + 4 + 3 + 3);
    }

    #[test]
    fn tick_by_zero() {
        assert_eq!(tick_by(0, &vec![1, 2, 3, 4, 5]), 0);
    }

    #[test]
    fn bsearch_normal() {
        assert_eq!(
            bsearch(0, MAX_TICKS, |t| tick_by(t, &vec![1, 2, 3, 4, 5]), 22),
            8
        );
    }

    #[test]
    fn bsearch_finds_max() {
        let ds = vec![1, 2, 3, 4, 5];

        for children in 1..2_000 {
            let t = bsearch(0, MAX_TICKS, |t| tick_by(t, &ds), children);

            assert!(tick_by(t, &ds) < children);
            assert!(children <= tick_by(t + 1, &ds));
        }
    }

    #[test]
    fn empty_rides_after_normal() {
        assert_eq!(
            empty_rides_after(8, &vec![1, 2, 3, 4, 5]).collect::<Vec<bool>>(),
            vec![true, true, false, true, false]
        );
    }

    #[test]
    fn last_ride_normal() {
        assert_eq!(last_ride(22, 8, &vec![1, 2, 3, 4, 5]), 4);
    }

    #[test]
    fn bsearch_ticks() {
        let ds = vec![30, 30, 30, 30, 30];

        assert_eq!(0, bsearch(0, MAX_TICKS, |t| tick_by(t, &ds), 1));
        assert_eq!(0, bsearch(0, MAX_TICKS, |t| tick_by(t, &ds), 5));
        assert_eq!(30, bsearch(0, MAX_TICKS, |t| tick_by(t, &ds), 6));
        assert_eq!(30, bsearch(0, MAX_TICKS, |t| tick_by(t, &ds), 10));
        assert_eq!((2_000_000_000 / 5 - 1) * 30, bsearch(0, MAX_TICKS, |t| tick_by(t, &ds), 2_000_000_000));
    }

    #[test]
    fn even() {
        let ds = vec![1, 2, 3, 4, 5];
        let children = 137;

        let t = bsearch(0, MAX_TICKS, |t| tick_by(t, &ds), children);

        assert_eq!(t, 59);
        assert_eq!(last_ride(children, t, &ds), 1);
    }

    #[test]
    fn one_man() {
        let ds = vec![1, 2, 3, 4, 5];
        let children = 1;

        let t = bsearch(0, MAX_TICKS, |t| tick_by(t, &ds), children);

        assert_eq!(t, 0);
        assert_eq!(last_ride(children, t, &ds), 1);
    }

    #[test]
    fn short_line() {
        let ds = vec![1, 2, 3, 4, 5];
        let children = 6;

        let t = bsearch(0, MAX_TICKS, |t| tick_by(t, &ds), children);

        assert_eq!(t, 1);
        assert_eq!(last_ride(children, t, &ds), 1);
    }
}
