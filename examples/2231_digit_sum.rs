/// n=1일 수 있는 상황에서 아무 생각없이 (0..=54)를 빼고있는 날 보면서, 난 정말 값의 범위에 대해서 무신경하다는걸 알았고, 좀 더
/// 신경쓰면서 코딩해야할 것 같다.
use std::error::Error;
use std::io;

fn gen(n: i32) -> i32 {
    n + n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .sum::<i32>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;

    let n: i32 = buf.trim().parse()?;

    for i in (0..=54).rev() {
        if n - i > 0 && gen(n - i) == n {
            println!("{}", n - i);
            return Ok(())
        }
    }

    println!("0");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(gen(1), 2);
    }
}
