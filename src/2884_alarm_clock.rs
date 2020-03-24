// 2884 alarm clock
use std::fmt::{self, Display};
use std::io;
use std::ops::SubAssign;

fn parse(s: &str) -> (i32, i32) {
    let mut s = s.trim().split(' ');

    (
        s.next().unwrap().parse().unwrap(),
        s.next().unwrap().parse().unwrap(),
    )
}

struct Time(i32);

impl Time {
    fn new(h: i32, m: i32) -> Self {
        Time(h * 60 + m)
    }

    fn display(&self) -> String {
        if self.0 == 24 * 60 {
            "0 0".into()
        } else {
            format!("{} {}", self.0 / 60, self.0 % 60)
        }
    }
}

impl SubAssign<i32> for Time {
    fn sub_assign(&mut self, minutes: i32) {
        self.0 = self.0 - minutes + if self.0 - minutes > 0 { 0 } else { 24 * 60 }
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let (h, m) = parse(&s);

    let mut t = Time::new(h, m);
    t -= 45;

    println!("{}", t);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_1() {
        assert_eq!(parse("10 10"), (10, 10));
        assert_eq!(parse("0 30"), (0, 30));
        assert_eq!(parse("23 40"), (23, 40));
    }

    #[test]
    fn examples() {
        let mut t = Time::new(10, 10);
        t -= 45;
        assert_eq!(t.display(), "9 25");

        let mut t = Time::new(0, 30);
        t -= 45;
        assert_eq!(t.display(), "23 45");

        let mut t = Time::new(23, 40);
        t -= 45;
        assert_eq!(t.display(), "22 55");
    }

    #[test]
    fn midnight() {
        let mut t = Time::new(0, 45);
        t -= 45;
        assert_eq!(t.display(), "0 0");
    }
}

// let mut s = s.trim().split(' ');

// let (h, m): (i32, i32) = (
//     s.next().unwrap().parse().unwrap(),
//     s.next().unwrap().parse().unwrap(),
// );

// let mut t = h * 60 + m;

// t = t - 45 + if t - 45 > 0 { 0 } else { 24 * 60 };

// if t == 24 * 60 {
//     println!("0 0")
// } else {
//     println!("{} {}", t / 60, t % 60)
// }
