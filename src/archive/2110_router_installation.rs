use std::io::{self, Read};

fn parse(s: &str) -> (i32, i32, Vec<i32>) {
    let mut s = s.lines();

    let mut fst = s.next().unwrap().split_whitespace();
    let n = fst.next().unwrap().parse().unwrap();
    let c = fst.next().unwrap().parse().unwrap();

    (n, c, s.map(|l| l.parse().unwrap()).collect())
}

/// Check whether it's possible to place routers further than given distance from each other
fn is_possible(distances: &Vec<i32>, target_distance: i32, routers: i32) -> bool {
    let mut distances = distances.iter();

    'router: for _ in 0..(routers - 1) {
        let mut current_distance = 0;

        while let Some(d) = distances.next() {
            current_distance += d;
            if current_distance >= target_distance {
                continue 'router;
            }
        }

        return false;
    }

    true
}

fn bsearch<F>(start: i32, end: i32, f: F) -> i32
where
    F: Fn(i32) -> bool,
{
    let middle = start + (end - start) / 2;

    if start == end {
        return start;
    }

    if start == middle {
        if f(end) {
            return end;
        } else {
            return start;
        }
    }

    if !f(middle) {
        bsearch(start, middle - 1, f)
    } else {
        bsearch(middle, end, f)
    }
}

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();

    let (_, c, mut houses) = parse(&s);
    houses.sort();

    let distances = houses
        .windows(2)
        .map(|hs| hs[1] - hs[0])
        .collect::<Vec<i32>>();

    let max = bsearch(0, 1_000_000_000, |target_distance| {
        is_possible(&distances, target_distance, c)
    });

    println!("{}", max);
}
