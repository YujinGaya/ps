/// 팰린드롬 문장
/// https://www.acmicpc.net/problem/1054
///
/// 조건문을 줄이기 위해서 고민을 많이 했다.
///
/// 여전히 Prefix(""), Suffix("") 같은 가져선 안될 값을 표현할 수 있는 문제는 불편하다..
///
/// stack을 써서 풀어보려고 했는데, 재귀의 소중함을 깨달았다.
///
use std::collections::HashMap;
use std::io::{self, Read};

#[derive(PartialEq, Eq, Hash)]
enum Affix<'a> {
    Prefix(&'a [u8]),
    Suffix(&'a [u8]),
    Empty,
}

use Affix::*;

impl<'a> Affix<'a> {
    fn push(&self, word: &'a [u8]) -> Option<Self> {
        let (prefix, suffix): (&[u8], &[u8]) = match self {
            Prefix(prefix) => (prefix, word),
            Suffix(suffix) => (word, suffix),
            Empty => (word, b""),
        };

        if !prefix
            .iter()
            .zip(suffix.iter().rev())
            .all(|(b1, b2)| b1 == b2)
        {
            return None;
        }

        Some(if prefix.len() > suffix.len() {
            Prefix(&prefix[suffix.len()..])
        } else if prefix.len() < suffix.len() {
            Suffix(&suffix[..suffix.len() - prefix.len()])
        } else {
            Empty
        })
    }

    fn is_palindrome(&self) -> bool {
        let s = match self {
            Prefix(s) => s,
            Suffix(s) => s,
            Empty => return true,
        };

        s.iter().eq(s.iter().rev())
    }
}

fn count<'a>(
    k: (Affix<'a>, u16),
    words: &[&'a [u8]],
    memo: &mut HashMap<(Affix<'a>, u16), i64>,
) -> i64 {
    if let Some(&i) = memo.get(&k) {
        return i;
    }

    let (affix, used) = &k;
    let count = words
        .iter()
        .enumerate()
        .filter(|(i, _)| 1u16 << i & used == 0)
        .map(|(i, &word)| {
            if let Some(affix) = affix.push(word) {
                count((affix, used | 1 << i), words, memo)
            } else {
                0
            }
        })
        .sum::<i64>()
        + if affix.is_palindrome() { 1 } else { 0 };

    memo.insert(k, count);

    count
}

fn main() {
    let mut buf = [0; 3 + 14 * 13];
    let n = io::stdin().read(&mut buf).unwrap();
    let buf = &buf[0..n];

    let words: Vec<&[u8]> = buf
        .split(|&c| c == b'\n')
        .skip(1)
        .take_while(|w| !w.is_empty())
        .collect();

    println!("{}", count((Empty, 0), &words, &mut HashMap::new()) - 1);
}
