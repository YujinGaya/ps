use std::error::Error;
use std::io::{self, Read};

#[derive(Copy, Clone, Debug)]
enum Item {
    Char(char),
    BombCursor(usize),
}

use Item::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    let mut lines = buf.lines();
    let s = lines.next().unwrap().chars().collect::<Vec<char>>();
    let bomb = lines.next().unwrap().chars().collect::<Vec<char>>();

    let s = if bomb.len() == 1 {
        s.iter().filter(|&&c| c != bomb[0]).collect::<String>()
    } else {
        let mut stack = Vec::new();

        for ch in s {
            if let Some(&BombCursor(cursor)) = stack.last() {
                if bomb[cursor + 1] == ch {
                    stack.pop();
                    if cursor + 1 != bomb.len() - 1 {
                        stack.push(BombCursor(cursor + 1))
                    }
                    continue;
                }
            }

            if bomb[0] == ch {
                stack.push(BombCursor(0))
            } else {
                stack.push(Char(ch))
            }
        }

        stack
            .into_iter()
            .map(|item| match item {
                Char(ch) => ch.to_string(),
                BombCursor(cursor) => bomb[0..=cursor].iter().collect::<String>(),
            })
            .collect::<Vec<String>>()
            .join("")
    };

    println!("{}", if s.len() == 0 { "FRULA" } else { &s });

    Ok(())
}
