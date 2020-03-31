/// 불 끄기
/// https://www.acmicpc.net/problem/14939
///
/// # 아이디어
/// 스위치를 켜고 끄는 operation은 xor인데 associative, commutative하다.
/// 10x10의 스위치를 켜고 끄는 operation도 마찬가지로 associative, commutative하다.
/// 따라서 같은 스위치를 두 번 이상 켜고 끈다면 연산 순서를 바꿔서 없었던 일로 만들 수 있다.
/// 따라서 각 스위치를 두 번 이상 만지는 것은 의미가 없고, solution space는 각 스위치를 한번 조작하거나, 말거나로
/// 2^(10*10)으로 줄어든다.
///
/// 여기서 맨 첫 줄의 스위치를 켤지 끌지 임의로 결정하게 되면, 첫번째 줄의 전구를 끄기 위해서 영향을 줄 수 있는 스위치는 두 번째
/// 줄에 있는 스위치들 밖에 없다. 이때 각 행의 전구를 해당 행 다음 열의 스위치 하나만이 영향을 줄 수 있기 때문에 다음 열의 스위치를
/// 누를지 말지 하나로 결정된다. 이를 반복하면 남은 9열의 스위치를 모두 결정할 수 있는데, 이 스위치의 작동 결과 마지막 열의 전구가
/// 모두 꺼져있는 경우 그 때 쓴 스위치의 수를 출력한다. 0열의 1024개의 경우의 수가 9열의 1024개의 경우의 수로 1대1 대응이 되기
/// 때문에 정답은 언제나 하나이다.
/// 
/// # 더 나아갈 점
/// 어쩌면 1024개의 경우를 브루트포스 하지 않고 계산할 수 있을 것 같기도..?
///
use std::error::Error;
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
struct Bulbs([[bool; 10]; 10]);

impl FromStr for Bulbs {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut b = [[false; 10]; 10];
        let s: Vec<_> = s.chars().collect();
        for i in 0..100 {
            b[i / 10][i % 10] = s[i + i / 10] == 'O';
        }

        Ok(Bulbs(b))
    }
}

impl Bulbs {
    fn switch(&mut self, row: usize, col: usize) {
        if row != 0 {
            self.0[row - 1][col] ^= true;
        }

        if row != 9 {
            self.0[row + 1][col] ^= true;
        }

        if col != 0 {
            self.0[row][col - 1] ^= true;
        }

        if col != 9 {
            self.0[row][col + 1] ^= true;
        }

        self.0[row][col] ^= true;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();

    let bulbs: Bulbs = buf.parse().unwrap();
    let switches: Bulbs = Bulbs([[false; 10]; 10]);

    for seed in 0..1024 {
        let mut switches = switches.clone();
        let mut bulbs = bulbs.clone();

        // possible 1024 cases of first row
        for col in 0..10 {
            switches.0[0][col] = (seed >> col) % 2 == 0;
            if switches.0[0][col] {
                bulbs.switch(0, col);
            }
        }

        // solve here
        for row in 1..10 {
            for col in 0..10 {
                if bulbs.0[row - 1][col] {
                    switches.0[row][col] = true;
                    bulbs.switch(row, col);
                }
            }
        }

        println!("{:?}", blubs.0[9]);

        if bulbs.0[9].iter().all(|b| !b) {
            println!(
                "{}",
                switches
                    .0
                    .iter()
                    .map(|row| row.iter())
                    .flatten()
                    .filter(|&&s| s)
                    .count()
            );
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "#O########\n\
        OOO#######\n\
        #O########\n\
        ####OO####\n\
        ###O##O###\n\
        ####OO####\n\
        ##########\n\
        ########O#\n\
        #######OOO\n\
        ########O#";

    #[test]
    fn parse() {
        assert_eq!(
            EXAMPLE.parse::<Bulbs>().unwrap(),
            Bulbs([
                [false, true, false, false, false, false, false, false, false, false],
                [true, true, true, false, false, false, false, false, false, false],
                [false, true, false, false, false, false, false, false, false, false],
                [false, false, false, false, true, true, false, false, false, false],
                [false, false, false, true, false, false, true, false, false, false],
                [false, false, false, false, true, true, false, false, false, false],
                [false, false, false, false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false, false, true, false],
                [false, false, false, false, false, false, false, true, true, true],
                [false, false, false, false, false, false, false, false, true, false],
            ])
        );
    }

    #[test]
    fn switch() {
        let mut b = EXAMPLE.parse::<Bulbs>().unwrap();

        b.switch(1, 1);

        assert_eq!(
            b,
            Bulbs([
                [false, false, false, false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false, false, false, false],
                [false, false, false, false, true, true, false, false, false, false],
                [false, false, false, true, false, false, true, false, false, false],
                [false, false, false, false, true, true, false, false, false, false],
                [false, false, false, false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false, false, true, false],
                [false, false, false, false, false, false, false, true, true, true],
                [false, false, false, false, false, false, false, false, true, false],
            ])
        );

        b.switch(9, 9);

        assert_eq!(
            b,
            Bulbs([
                [false, false, false, false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false, false, false, false],
                [false, false, false, false, true, true, false, false, false, false],
                [false, false, false, true, false, false, true, false, false, false],
                [false, false, false, false, true, true, false, false, false, false],
                [false, false, false, false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false, false, true, false],
                [false, false, false, false, false, false, false, true, true, false],
                [false, false, false, false, false, false, false, false, false, true],
            ])
        );
    }
}
