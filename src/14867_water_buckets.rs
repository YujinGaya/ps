use std::collections::{HashSet, VecDeque};
use std::io;

enum Bucket {
    A,
    B,
}

enum Direction {
    FromAToB,
    FromBToA,
}

use Bucket::*;
use Direction::*;

#[derive(Clone, Copy, Debug)]
struct Buckets {
    a_capacity: i32,
    b_capacity: i32,
    a: i32,
    b: i32,
}

impl Buckets {
    fn new(a_capacity: i32, b_capacity: i32, a: i32, b: i32) -> Self {
        Buckets {
            a_capacity,
            b_capacity,
            a,
            b,
        }
    }

    fn fill(mut self, bucket: Bucket) -> Self {
        match bucket {
            A => self.a = self.a_capacity,
            B => self.b = self.b_capacity,
        }

        self
    }

    fn empty(mut self, bucket: Bucket) -> Self {
        match bucket {
            A => self.a = 0,
            B => self.b = 0,
        }

        self
    }

    fn pour(mut self, direction: Direction) -> Self {
        match direction {
            FromAToB => {
                if self.b_capacity - self.b > self.a {
                    self.b = self.b + self.a;
                    self.a = 0;
                } else {
                    self.a = self.a - (self.b_capacity - self.b);
                    self.b = self.b_capacity;
                }
            }
            FromBToA => {
                if self.a_capacity - self.a > self.b {
                    self.a = self.a + self.b;
                    self.b = 0;
                } else {
                    self.b = self.b - (self.a_capacity - self.a);
                    self.a = self.a_capacity;
                }
            }
        }

        self
    }

    fn get(&self) -> (i32, i32) {
        (self.a, self.b)
    }
}

fn parse(s: &str) -> (i32, i32, i32, i32) {
    let mut s = s.split(' ');

    (
        s.next().unwrap().parse().unwrap(),
        s.next().unwrap().parse().unwrap(),
        s.next().unwrap().parse().unwrap(),
        s.next().unwrap().parse().unwrap(),
    )
}

struct Tree {
    queue: VecDeque<((i32, i32), i32)>,
    set: HashSet<(i32, i32)>,
}

impl Tree {
    fn new() -> Self {
        Tree {
            queue: VecDeque::new(),
            set: HashSet::new(),
        }
    }

    fn push(&mut self, (buckets, depth): ((i32, i32), i32)) {
        if self.set.insert(buckets) {
            self.queue.push_back((buckets, depth));
        }
    }

    fn next(&mut self) -> Option<((i32, i32), i32)> {
        self.queue.pop_front()
    }
}

fn find_min((a_capacity, b_capacity, a_target, b_target): (i32, i32, i32, i32)) -> i32 {
    let mut tree = Tree::new();
    tree.push(((0, 0), 0));

    loop {
        if let Some(((a, b), depth)) = tree.next() {
            if (a, b) == (a_target, b_target) {
                break depth;
            }

            let buckets = Buckets::new(a_capacity, b_capacity, a, b);

            tree.push((buckets.fill(A).get(), depth + 1));
            tree.push((buckets.fill(B).get(), depth + 1));
            tree.push((buckets.empty(A).get(), depth + 1));
            tree.push((buckets.empty(B).get(), depth + 1));
            tree.push((buckets.pour(FromAToB).get(), depth + 1));
            tree.push((buckets.pour(FromBToA).get(), depth + 1));
        } else {
            break -1;
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let depth = find_min(parse(input.trim_end()));

    println!("{}", depth);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn a2b4_from_a2b5_in_8() {
        assert_eq!(
            Buckets::new(2, 5, 0, 0)
                .fill(B)
                .pour(FromBToA)
                .empty(A)
                .pour(FromBToA)
                .empty(A)
                .pour(FromBToA)
                .fill(B)
                .pour(FromBToA)
                .get(),
            (2, 4)
        )
    }

    #[test]
    fn a2b4_from_a2b5_in_5() {
        assert_eq!(
            Buckets::new(2, 5, 0, 0)
                .fill(A)
                .pour(FromAToB)
                .fill(A)
                .pour(FromAToB)
                .fill(A)
                .get(),
            (2, 4)
        )
    }

    #[test]
    fn parse_1() {
        assert_eq!(parse("3 7 3 2"), (3, 7, 3, 2));
    }

    #[test]
    fn parse_2() {
        assert_eq!(parse("2 5 0 1"), (2, 5, 0, 1));
    }

    #[test]
    fn find_min_1() {
        assert_eq!(find_min((3, 7, 3, 2)), 9);
    }

    #[test]
    fn find_min_2() {
        assert_eq!(find_min((2, 5, 0, 1)), 5);
    }

    #[test]
    fn find_min_3() {
        assert_eq!(find_min((3, 5, 2, 4)), -1);
    }

    #[test]
    fn find_min_zero() {
        assert_eq!(find_min((3, 5, 0, 0)), 0);
    }

    #[test]
    fn find_min_loop() {
        assert_eq!(find_min((50_515, 1_595, 69_099, 17)), -1);
    }
}
