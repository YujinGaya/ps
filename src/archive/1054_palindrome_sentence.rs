/// 팰린드롬 문장
/// https://www.acmicpc.net/problem/1054
/// 
/// `unsafe`, `static`을 이용해서 풀어도 보고, lifetime parameter가 덕지덕지 붙은 `struct`를 없애서 코드를 간결하게
/// 만들어보기도 하고, 러스트에 대해서 이것저것 많이 배웠다.
/// 
/// 아이디어를 내는데 시간이 굉장히 오래 걸렸다. 시간복잡도를 제대로 분석 못해서 확신이 없는게 컸던 것 같다.
///
use std::collections::HashMap;
use std::io::{self, Read};

fn is_palindrome(start: &[u8], end: &[u8]) -> bool {
    match (start.len(), end.len()) {
        (0, 0) => false,
        (s, e) if s > e => (0..((s - e) / 2)).all(|i| start[e + i] == start[s - i - 1]),
        (s, e) => (0..((e - s) / 2)).all(|i| end[s + i] == end[e - i - 1]),
    }
}

fn count(
    (start, end, used): (Vec<u8>, Vec<u8>, u16),
    words: &[&[u8]],
    memo: &mut HashMap<(bool, Vec<u8>, u16), i64>,
) -> i64 {
    let (s, e) = (start.len(), end.len());
    let k = (
        s >= e,
        if s > e { &start[e..] } else if s == e { &[] } else { &end[s..] }.to_vec(),
        used,
    );

    if let Some(&i) = memo.get(&k) {
        return i;
    }

    let count = words
        .iter()
        .enumerate()
        .filter(|(i, _)| 1u16 << i & used == 0)
        .filter(|(_, &word)| {
            if s > e {
                (0..word.len().min(s - e)).all(|i| start[e + i] == word[word.len() - i - 1])
            } else {
                let len = word.len().min(e - s);
                end[s..s + len] == word[..len]
            }
        })
        .map(|(i, &word)| {
            if s > e {
                let mut end = end.clone();
                end.extend(word.iter().rev());
                (start.clone(), end, used | 1 << i)
            } else {
                ([&start, word].concat(), end.clone(), used | 1 << i)
            }
        })
        .map(|w| count(w, words, memo))
        .sum::<i64>()
        + if is_palindrome(&start, &end) { 1 } else { 0 };

    memo.insert(k, count);

    count
}

fn main() {
    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf).unwrap();

    let words: Vec<&[u8]> = buf
        .split(|&c| c == b'\n')
        .skip(1)
        .take_while(|w| !w.is_empty())
        .collect();

    println!("{}", count(Default::default(), &words, &mut HashMap::new()));
}
