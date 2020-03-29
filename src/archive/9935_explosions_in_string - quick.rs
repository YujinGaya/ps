/// 문자열 폭발
/// https://www.acmicpc.net/problem/9935
///
/// # 아이디어: in-place로 공간 효율 높이기
/// 스택을 쓰게 되면 스택을 위한 공간이 추가로 필요한데에 비해 in-place로 작업해서 메모리와 속도가 네 배 정도 좋아졌다.
/// (63928kb -> 18956kb, 88ms -> 16ms)
///
/// # 좋은 점: 자연스러운 문자열 비교
/// C에서 strncmp 함수로 부자연스럽게 문자열을 비교해야하는 것과 다르게, Rust에서는 &str 이나 &[char]를 == 연산자로 비교할
/// 수 있다.
///
/// # 한계: 스타일
/// Rust는 usize 연산에서 오버플로우가 일어나면 패닉하기 때문에 (2)처럼 커서를 부자연스럽게 조절해야하는 문제가 있었다. end가
/// "현재까지 확정된 문자열의 끝 인덱스"로 생각하게 되면 찾으려고 하는 문자열의 prefix가 explosions인 경우에 확정된 문자열이
/// 없는 경우, 즉 끝 인덱스도 정의할 수 없는 상태가 된다. C였다면 잠시 오버플로우가 일어났다가 loop의 끝에 index를 1씩 더할 때
/// 다시 0이 될 문제인데, Rust에서는 패닉이 일어나 아래처럼 구현했다.
///
/// # 한계: 소유권 우회
/// (3)은 for ch in s.iter()로 쓸 수도 있었다. 하지만 그렇게 되면 s의 소유권에 문제가 생긴다. s.iter()에서
/// immutable하게 빌린 도중에 s[end] = ch; 로 mutable하게 빌릴 수는 없기 때문이다. 이를 우회하려고 c style for loop를
/// 썼다.
///
use std::error::Error;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    let mut lines = buf.lines();
    let mut s = lines.next().unwrap().chars().collect::<Vec<char>>();
    let bomb = lines.next().unwrap().chars().collect::<Vec<char>>();

    // End of valid output
    let mut end = 0;

    // - (3)
    for cur in 0..s.len() {
        s[end] = s[cur];

        end = if end + 1 >= bomb.len() && &s[(end + 1 - bomb.len())..=end] == &bomb[..] {
            // - (1)
            end + 1 - bomb.len() // - (2)
        } else {
            end + 1
        };
    }

    s.truncate(end);

    let s = s.iter().collect::<String>();
    println!("{}", if s.len() == 0 { "FRULA" } else { &s });

    Ok(())
}
