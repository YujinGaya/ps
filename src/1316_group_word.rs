// https://www.acmicpc.net/problem/1316

use std::io::{self, Read};

fn read(s: &str) -> Option<(i32, Vec<&str>)> {
    let mut s = s.lines();

    Some((s.next()?.parse().unwrap(), s.collect()))
}

fn is_group(s: &str) -> bool {
    let s = s.chars().fold(
        Vec::new(),
        |mut vec, c| {
            if *vec.last().unwrap_or(&' ') != c {
                vec.push(c);
            }

            vec
        }
    );

    for i in 0..s.len() {
        for j in i + 1..s.len() {
            if s[i] == s[j] {
                return false
            }
        }
    }

    true
}

fn count_group(words: Vec<&str>) -> usize {
    words.iter().filter(|word| is_group(word)).count()
}

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();

    let (_, words) = read(&s).unwrap();

    let count = count_group(words);

    println!("{}", count);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_1() {
        assert_eq!(
            read("3\nhappy\nnew\nyear"),
            Some((3, vec!["happy", "new", "year"]))
        )
    }

    #[test]
    fn read_2() {
        assert_eq!(
            read("4\naba\nabab\nabcabc\na"),
            Some((4, vec!["aba", "abab", "abcabc", "a"]))
        )
    }

    #[test]
    fn is_group_1() {
        assert!(is_group("happy"))
    }

    #[test]
    fn is_group_2() {
        assert!(is_group("new"))
    }

    #[test]
    fn is_group_3() {
        assert!(is_group("year"))
    }

    #[test]
    fn is_group_4() {
        assert!(!is_group("aba"))
    }

    #[test]
    fn is_group_5() {
        assert!(!is_group("abab"))
    }

    #[test]
    fn is_group_6() {
        assert!(!is_group("abcabc"))
    }

    #[test]
    fn is_group_7() {
        assert!(is_group("a"))
    }

    #[test]
    fn count_group_1() {
        assert_eq!(count_group(vec!["happy", "new", "year"]), 3);
    }

    #[test]
    fn count_group_2() {
        assert_eq!(count_group(vec!["aba", "abab", "abcabc", "a"]), 1);
    }
}
