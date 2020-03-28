// #10523 find lines
use std::io::{self, Read};

fn parse(s: &str) -> (i32, i32, Vec<(i32, i32)>) {
    let mut s = s.lines();

    let n = s.next().unwrap().parse().unwrap();
    let p = s.next().unwrap().parse().unwrap();

    (
        n,
        p,
        s.map(|l| {
            let mut l = l.split_whitespace();
            (
                l.next().unwrap().parse().unwrap(),
                l.next().unwrap().parse().unwrap(),
            )
        })
        .collect(),
    )
}

fn target_length(n: i32, p: i32) -> i32 {
    n * p / 100 + if n * p % 100 == 0 { 0 } else { 1 }
}

fn is_pts_in_line(p0: (i32, i32), p1: (i32, i32), p2: (i32, i32)) -> bool {
    (p2.0 - p0.0) as i64 * (p1.1 - p0.1) as i64 == (p1.0 - p0.0) as i64 * (p2.1 - p0.1) as i64
}

fn find_all_lns(pts: &[(i32, i32)], target: i32) -> Vec<((i32, i32), (i32, i32), i32)> {
    let mut v = Vec::new();

    for i in 0..pts.len() {
        for j in (i + 1)..pts.len() {
            let mut length = 2;

            for k in (j + 1)..pts.len() {
                if is_pts_in_line(pts[i], pts[j], pts[k]) {
                    length += 1;
                }
            }

            if length >= target
                && v.iter().all(|&(p0, p1, _)| {
                    !(is_pts_in_line(pts[i], pts[j], p0) && is_pts_in_line(pts[j], p0, p1))
                })
            {
                v.push((pts[i], pts[j], length));
            }
        }
    }

    v
}

fn elongate_lns(
    mut lns: Vec<((i32, i32), (i32, i32), i32)>,
    pts: &[(i32, i32)],
    target: i32,
) -> Vec<((i32, i32), (i32, i32), i32)> {
    for &mut (p0, p1, ref mut length) in &mut lns {
        for &p2 in pts {
            if is_pts_in_line(p0, p1, p2) {
                *length += 1;
            }
        }
    }

    lns.retain(|&(_, _, length)| length >= target);

    lns
}

fn merge_lns(
    mut l: Vec<((i32, i32), (i32, i32), i32)>,
    r: Vec<((i32, i32), (i32, i32), i32)>,
) -> Vec<((i32, i32), (i32, i32), i32)> {
    for &(p0, p1, length) in &r {
        if l.iter()
            .all(|&(p2, p3, _)| !(is_pts_in_line(p0, p1, p2) && is_pts_in_line(p1, p2, p3)))
        {
            l.push((p0, p1, length));
        }
    }

    l
}

fn find(pts: &[(i32, i32)], length: i32) -> Vec<((i32, i32), (i32, i32), i32)> {
    if length < 6 {
        find_all_lns(pts, length)
    } else {
        let mid = pts.len() / 2;
        let half = length / 2 + if length % 2 == 0 { 0 } else { 1 };

        let l_pts = &pts[0..mid];
        let r_pts = &pts[mid..pts.len()];

        let l_lns = find(l_pts, half);
        let r_lns = find(r_pts, half);

        let l_lns = elongate_lns(l_lns, r_pts, length);
        let r_lns = elongate_lns(r_lns, l_pts, length);

        merge_lns(l_lns, r_lns)
    }
}

fn possible(pts: &[(i32, i32)], length: i32) -> bool {
    if pts.len() == 1 {
        return true;
    }

    !find(pts, length).is_empty()
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let (n, p, pts) = parse(&buf);

    let length = target_length(n, p);

    println!(
        "{}",
        if possible(&pts, length) {
            "possible"
        } else {
            "impossible"
        }
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int_range() {
        assert!(is_pts_in_line(
            (1_000_000_000, 1_000_000_000),
            (0, 0),
            (1, 1)
        ));
    }

    #[test]
    fn target_test() {
        // 1 <= n <= 100_000
        // 20 <= p <= 100
        assert_eq!(target_length(5, 55), 3);
        assert_eq!(target_length(5, 45), 3);
        assert_eq!(target_length(5, 100), 5);
        assert_eq!(target_length(5, 99), 5);
        assert_eq!(target_length(5, 81), 5);
        assert_eq!(target_length(5, 80), 4);
        assert_eq!(target_length(100_000, 100), 100_000);
        assert_eq!(target_length(100_000, 20), 20_000);
        assert_eq!(target_length(99_999, 20), 20_000);
        assert_eq!(target_length(99_996, 20), 20_000);
        assert_eq!(target_length(99_995, 20), 19_999);
        assert_eq!(target_length(99_995, 21), 20_999);
    }

    #[test]
    fn alternate() {
        let alt: Vec<(i32, i32)> = (0..60_000)
            .map(|i| {
                (0..2)
                    .map(move |j| {
                        vec![(i, j), (i + 600_000, j + 10), (i + 100_000_000, j + 60_000)]
                            .into_iter()
                    })
                    .flatten()
            })
            .flatten()
            .collect();

        assert!(possible(&alt, 60_000));
        assert!(!possible(&alt, 60_001));
    }

    #[test]
    fn less_pts() {
        assert!(possible(&vec![(0, 0)], 1));
    }

    #[test]
    fn test_possible() {
        assert!(possible(
            &vec![(0, 0), (10, 10), (10, 0), (0, 10), (3, 3)],
            1
        ));

        assert!(possible(
            &vec![(0, 0), (10, 10), (10, 0), (0, 10), (3, 3)],
            2
        ));

        assert!(possible(
            &vec![(0, 0), (10, 10), (10, 0), (0, 10), (3, 3)],
            3
        ));

        assert!(!possible(
            &vec![(0, 0), (10, 10), (10, 0), (0, 10), (3, 3)],
            4
        ));

        assert!(!possible(
            &vec![(0, 0), (10, 10), (10, 0), (0, 10), (3, 3)],
            5
        ));

        assert!(possible(
            &vec![(0, 0), (10, 10), (10, 0), (0, 10), (3, 4)],
            2
        ));

        assert!(!possible(
            &vec![(0, 0), (10, 10), (10, 0), (0, 10), (3, 4)],
            3
        ));

        assert!(possible(
            &vec![
                (0, 0),
                (10, 10),
                (10, 0),
                (0, 10),
                (3, 4),
                (1, 1),
                (2, 2),
                (3, 3),
                (4, 4)
            ],
            6
        ));

        assert!(!possible(
            &vec![
                (0, 0),
                (0, 5),
                (0, 10),
                (5, 0),
                (5, 5),
                (5, 10),
                (10, 0),
                (10, 5),
                (10, 10),
                (3, 4),
                (1, 1),
                (2, 2),
                (3, 3),
                (4, 4)
            ],
            11
        ));
    }

    #[test]
    fn long() {
        assert!(possible(
            &(0..100_000).map(|i| (i, i)).collect::<Vec<(i32, i32)>>(),
            50_000
        ));
    }

    #[test]
    fn grid() {
        assert!(!possible(
            &(0..1000)
                .map(|i| (0..100).map(move |j| (i, j)))
                .flatten()
                .collect::<Vec<(i32, i32)>>(),
            20_000
        ));

        assert!(!possible(
            &(0..1000)
                .map(|i| (0..100).map(move |j| (i, j)))
                .flatten()
                .collect::<Vec<(i32, i32)>>(),
            20001
        ));

        assert!(!possible(
            &(0..1000)
                .map(|i| (0..100).map(move |j| (i, j)))
                .flatten()
                .collect::<Vec<(i32, i32)>>(),
            50001
        ));

        assert!(!possible(
            &(0..1000)
                .map(|i| (0..100).map(move |j| (i, j)))
                .flatten()
                .collect::<Vec<(i32, i32)>>(),
            100_000
        ));

        assert!(possible(
            &(0..20000)
                .map(|i| (0..5).map(move |j| (i, j)))
                .flatten()
                .collect::<Vec<(i32, i32)>>(),
            20_000
        ));
    }
}
