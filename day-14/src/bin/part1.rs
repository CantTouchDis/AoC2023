#![cfg_attr(feature = "unstable", feature(test))]

#[cfg(not(test))]
fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(part1(input));
}


fn tilt_north(board: &mut Vec<Vec<char>>) {
    let mut first_free = vec![0; board[0].len()];
    for y in 0..board.len() {
        for x in 0..board[y].len() {
            // do the stuff
            match board[y][x] {
                '#' => {
                    first_free[x] = y + 1;
                },
                'O' => {
                    // move to the first free pos
                    let current = board[y][x];
                    board[y][x] = board[first_free[x]][x];
                    board[first_free[x]][x] = current;
                    first_free[x] += 1;

                },
                _   => (),

            }
        }
    }

}

fn part1(input: &str) -> String {
    let mut board = input.lines().map(|c| c.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    tilt_north(&mut board);

    let result = board.iter().enumerate().map(|(i, l)| {
        l.iter().filter(|&c| *c == 'O').count() * (board.len() - i)
    }).sum::<usize>();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../../assets/test.txt").trim();
        assert_eq!(part1(input), "136");
    }
}

#[cfg(all(feature = "unstable", test))]
mod bench {
    use super::*;
    extern crate test;
    use self::test::Bencher;

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(|| {
            let input = test::black_box(include_str!("../../assets/input.txt").trim());
            let _ = part1(input);
        });
    }
}
