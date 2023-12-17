#![cfg_attr(feature = "unstable", feature(test))]

use std::collections::{BTreeSet, HashSet};

#[cfg(not(test))]
fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(part1(input));
}

#[derive(Ord, Eq, PartialEq, PartialOrd, Clone, Copy, Debug, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}


fn dir_to_next_pos(pos : (i32, i32), dir : Direction) -> (i32, i32) {
    if dir == Direction::Up {
        (pos.0 - 1, pos.1)
    }
    else if dir == Direction::Down {
        (pos.0 + 1, pos.1)
    }
    else if dir == Direction::Left {
        (pos.0, pos.1 - 1)
    }
    else {
        (pos.0, pos.1 + 1)
    }
}

fn part1(input: &str) -> String {
    let mut visited : HashSet::<((i32, i32), (Direction, i32))> = HashSet::new();
    // cost so far, position, straight steps
    let mut open_list : BTreeSet::<(i32, (i32, i32), (Direction, i32))> = BTreeSet::new();
    let board = input.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<i32>>()).collect::<Vec<_>>();
    
    let board_height = board.len() as i32;
    let board_width = board[0].len() as i32;

    open_list.insert((board[0][1], (0,1), (Direction::Right, 1)));
    open_list.insert((board[1][0], (1,0), (Direction::Down, 1)));

    while let Some(node) = open_list.pop_first() {
        //println!("{}", node.0);
        // found the exit
        if node.1 == (board_height - 1, board_width - 1) {
            return node.0.to_string();
        }
        // node was already visited with a smaller value
        if !visited.insert((node.1, node.2)) {
            continue;
        }
        // create the new neighbours
        for next_dir in [Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
            let next = dir_to_next_pos(node.1, next_dir);
            if next.0 < 0 || next.1 < 0 || next.0 >= board_height || next.1 >= board_width {
                continue;
            }
            if node.2.0 == Direction::Up && next_dir == Direction::Down ||
                node.2.0 == Direction::Down && next_dir == Direction::Up ||
                node.2.0 == Direction::Left && next_dir == Direction::Right ||
                node.2.0 == Direction::Right && next_dir == Direction::Left
            {
                continue;
            }
            let same_dir_steps = 
                if next_dir == node.2.0 {
                    node.2.1 + 1
                }
                else {
                    1
                };
            if same_dir_steps == 4 {
                continue;
            }
            open_list.insert((node.0 + board[next.0 as usize][next.1 as usize], next, (next_dir, same_dir_steps)));
        }
    }
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../../assets/test.txt").trim();
        assert_eq!(part1(input), "102");
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
