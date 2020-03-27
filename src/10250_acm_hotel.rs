use std::io::{self, Read};

fn parse(s: &str) -> (i32, Vec<(i32, i32, i32)>) {
    let mut s = s.lines();
    let n = s.next().unwrap().parse().unwrap();

    (
        n,
        s.map(|line| {
            let mut line = line.split_whitespace();
            (
                line.next().unwrap().parse().unwrap(),
                line.next().unwrap().parse().unwrap(),
                line.next().unwrap().parse().unwrap(),
            )
        })
        .collect(),
    )
}

fn room(h: i32, n: i32) -> String {
    format!(
        "{}{:02}",
        if n % h == 0 { h } else { n % h },
        n / h + if n % h == 0 { 0 } else { 1 }
    )
}

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();

    let (_, hotels) = parse(&s);
    hotels.iter().for_each(|&(h, _, n)| {
        println!("{}", room(h, n));
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn room_format() {
        for h in 1..99 {
            for n in 1..10 {
                println!("h: {}, n: {}: {}", h, n, room(h, n));
            }
        }
    }
}
