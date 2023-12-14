#![cfg_attr(feature = "unstable", feature(test))]

use std::collections::HashMap;
use std::hash::BuildHasher;

#[cfg(not(test))]
fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(part2(input));
}

fn rotate(v: &mut Vec<Vec<char>>){
    let n = v.len();
    for y in 0..(n/2) {
        for x in 0..(n/2) {
            let tmp : char = v[y][x];
            v[y][x] = v[n - 1 - x][y];
            v[n - 1 - x][y] = v[n - 1 - y][n - 1 - x];
            v[n - 1 - y][n - 1 - x] = v[x][n - 1 - y];
            v[x][n - 1 - y] = tmp;
        }
    }
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

fn tilt_cycle(board: &mut Vec<Vec<char>>){
    tilt_north(board);
    rotate(board);
    tilt_north(board);
    rotate(board);
    tilt_north(board);
    rotate(board);
    tilt_north(board);
    rotate(board);
}

fn part2(input: &str) -> String {
    let mut board = input.lines().map(|c| c.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    // contains only the 
    let mut seen_boards = HashMap::new();
    let hasher = std::collections::hash_map::RandomState::new();
    let mut remaining_steps = 0;
    let m = 1000000000;
    for i in 0..m {
        if let Some(index) = seen_boards.insert(hasher.hash_one(&board), i)
        {
            //we found a loop
            let loop_size = i - index;
            remaining_steps = m - i;
            remaining_steps -= (remaining_steps / loop_size) * loop_size;
            break;
        }
        tilt_cycle(&mut board);
    }
    for _i in 0..remaining_steps {
        tilt_cycle(&mut board);
    }

    // println!("{}",
    //          board.iter().map(|l| l.iter().collect::<String>()).reduce(|a, b| {
    //          a + "\n" + &b
    //          }).unwrap()
    // );


    let result = board.iter().enumerate().map(|(i, l)| {
        l.iter().filter(|&c| *c == 'O').count() * (board.len() - i)
    }).sum::<usize>();
    // compute score

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../../assets/test.txt").trim();
        assert_eq!(part2(input), "64");
    }
}

#[cfg(all(feature = "unstable", test))]
mod bench {
    use super::*;
    extern crate test;
    use self::test::Bencher;

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = test::black_box(include_str!("../../assets/input.txt").trim());
        b.iter(move || {
            //std::thread::sleep(std::time::Duration::from_nanos(1000));
            let _ = part2(input);
        });
    }
}
