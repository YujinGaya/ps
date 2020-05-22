// 17612 shopping mall

use std::convert::identity;
use std::io::{self, Read};

// @return (N, k, Vec<(id, w)>)
// N: # of customers, 1 <= N <= 100_000
// k: # of cashers, 1 <= k <= 100_000
// id: id of customer, 1 <= id <= 1_000_000
// w: # of items, 1 <= w <= 20
fn parse(s: &str) -> (usize, usize, Vec<(i32, i8)>) {
    let mut s = s.lines();

    let mut fst = s.next().unwrap().split(' ');

    let n = fst.next().unwrap().parse().unwrap();
    let k = fst.next().unwrap().parse().unwrap();

    let customers = s
        .map(|l| {
            let mut l = l.split(' ');
            (
                l.next().unwrap().parse().unwrap(),
                l.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    (n, k, customers)
}

fn run(k: usize, waiting: Vec<(i32, i8)>) -> Vec<i32> {
    let mut waiting = waiting.into_iter().peekable();
    let mut cashiers = vec![None; k]; // Option<(id, time remaining)>
    let mut finished = Vec::new();

    while waiting.peek().is_some() {
        // send waiting customers to idle cashiers, and tick
        let mut waiting_empty = false;

        for cashier in cashiers.iter_mut() {
            if cashier.is_none() {
                match waiting.next() {
                    Some((id, w)) => *cashier = Some((id, w)),
                    None => waiting_empty = true,
                }
            }

            if cashier.unwrap_or((0, 0)).1 > 0 {
                *cashier = cashier.map(|(id, w)| (id, w - 1));
            }
        }

        for cashier in cashiers.iter_mut().rev() {
            if let &mut Some((id, w)) = cashier {
                if w == 0 {
                    finished.push(id);
                    *cashier = None;
                }
            }
        }

        if waiting_empty { break }
    }

    let mut cashiers: Vec<(i32, i8)> = cashiers.into_iter().filter_map(identity).rev().collect();

    cashiers.sort_by_key(|c| c.1);

    [finished, cashiers.iter().map(|c| c.0).collect()].concat()
}

fn fold(v: Vec<i32>) -> i64 {
    v.iter()
        .enumerate()
        .fold(0i64, |sum, (i, &id)| sum + (i as i64 + 1) * id as i64)
}

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let (_, k, waiting) = parse(&s);

    println!("{}", fold(run(k, waiting)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example_1() {
        assert_eq!(
            parse("10 3\n123 4\n21 5\n34 14\n56 1\n45 7\n723 5\n55 7\n13 5\n910 10\n73 3"),
            (
                10,
                3,
                vec![
                    (123, 4),
                    (21, 5),
                    (34, 14),
                    (56, 1),
                    (45, 7),
                    (723, 5),
                    (55, 7),
                    (13, 5),
                    (910, 10),
                    (73, 3)
                ]
            )
        )
    }

    #[test]
    fn fold_1() {
        assert_eq!(fold(vec![1, 1, 1, 1]), 10);
        assert_eq!(fold(vec![1, 2, 3, 4]), 30);
        assert_eq!(fold(vec![4, 3, 2, 1]), 20);
        assert_eq!(fold((1..=1000).collect::<Vec<i32>>()), 1000 * 1001 * 2001 / 6);
    }

    #[test]
    fn example_1() {
        let k = 3;
        let waiting = vec![
            (123, 4),
            (21, 5),
            (34, 14),
            (56, 1),
            (45, 7),
            (723, 5),
            (55, 7),
            (13, 5),
            (910, 10),
            (73, 3),
        ];
        let finished = run(k, waiting);
        assert_eq!(finished, vec![123, 21, 56, 723, 45, 34, 55, 13, 73, 910]);
        assert_eq!(fold(finished), 13900);
    }

    #[test]
    fn example_finish_together() {
        let k = 3;
        let waiting = vec![
            (123, 4),
            (21, 5),
            (34, 14),
            (56, 1),
            (45, 7),
            (723, 5),
            (55, 7),
            (13, 5),
            (910, 10),
            (1000, 3),
            (73, 3),
        ];
        let finished = run(k, waiting);
        assert_eq!(
            finished,
            vec![123, 21, 56, 723, 45, 34, 55, 13, 73, 1000, 910]
        );
        assert_eq!(fold(finished), 13900 + 1000 * 10 + 910);
    }

    #[test]
    fn one_cashier() {
        let k = 1;
        let waiting = vec![(1, 1), (2, 1), (3, 1), (4, 1)];
        let finished = run(k, waiting);
        assert_eq!(finished, vec![1, 2, 3, 4]);
        assert_eq!(fold(finished), 30);
    }

    #[test]
    fn one_customer() {
        let k = 3;
        let waiting = vec![(1, 1)];
        let finished = run(k, waiting);
        assert_eq!(finished, vec![1]);
        assert_eq!(fold(finished), 1);
    }

    #[test]
    fn one_customer_one_cashier() {
        let k = 1;
        let waiting = vec![(1, 1)];
        let finished = run(k, waiting);
        assert_eq!(finished, vec![1]);
        assert_eq!(fold(finished), 1);
    }

    #[test]
    fn max_customer() {
        let k = 1;
        let waiting = (1..=100_000).map(|i| (i, 1)).collect();
        let finished = run(k, waiting);
        assert_eq!(finished, (1..=100_000).collect::<Vec<i32>>());
        assert_eq!(fold(finished), 100_000 * (100_000 + 1) * (2 * 100_000 + 1) / 6);
    }
}
