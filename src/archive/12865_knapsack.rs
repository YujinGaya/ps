use std::io::{self, Read};

fn parse(s: &str) -> (usize, usize, Vec<(usize, usize)>) {
    let mut s = s.lines();

    let mut fst = s.next().unwrap().split_whitespace();
    let n = fst.next().unwrap().parse().unwrap();
    let k = fst.next().unwrap().parse().unwrap();

    (
        n,
        k,
        s.map(|l| {
            ({
                let mut l = l.split_whitespace();
                (
                    l.next().unwrap().parse().unwrap(),
                    l.next().unwrap().parse().unwrap(),
                )
            })
        })
        .collect(),
    )
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let (n, k, items) = parse(&buf);

    let mut memo = vec![0; (n + 1) * (k + 1)];

    for i in 1..=n {
        for w in 1..=k {
            let w_i = items[i - 1].0;

            memo[i * (k + 1) + w] = if w_i > w {
                memo[(i - 1) * (k + 1) + w]
            } else {
                memo[(i - 1) * (k + 1) + w].max(memo[(i - 1) * (k + 1) + w - w_i] + items[i - 1].1)
            }
        }
    }

    println!("{}", memo.last().unwrap());
}
