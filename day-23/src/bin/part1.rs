#![cfg_attr(feature = "unstable", feature(test))]

use std::collections::{HashMap, HashSet};

#[cfg(not(test))]
fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(part1(input));
}


#[derive(Debug)]
struct Board {
    blocked: HashSet<(i32, i32)>,
    slopes: HashMap<(i32, i32), char>,
    start_pos: (i32, i32),
    end_pos: (i32, i32),
}


fn part1(input: &str) -> String {
    let board = {
        let start_pos_x = input.lines().next().unwrap().chars().position(|c| c == '.').unwrap() as i32;
        let blocked = input.lines().enumerate().flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some((y as i32, x as i32))
                }
                else {
                    None
                }
            })
        }).collect::<HashSet<(i32, i32)>>();
        let slopes = input.lines().enumerate().skip(1).flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| {
                if c != '#' && c != '.' {
                    Some(((y as i32, x as i32), c))
                }
                else {
                    None
                }
            })
        }).collect::<HashMap<(i32, i32), char>>();
        let (end_pos_y, last_line) = input.lines().enumerate().last().unwrap();
        let end_pos_x = last_line.chars().position(|c| c == '.').unwrap();
        Board { blocked, slopes, start_pos: (0, start_pos_x), end_pos: (end_pos_y as i32, end_pos_x as i32) }
    };
    let slope : HashMap<char, (i32, i32)> = HashMap::from(
        [
        ('v', (1, 0)),
        ('>', (0, 1)),
        ('^', (-1, 0)),
        ('<', (0, -1)),
        ]
        );


    // for each slope compute the longest path to outgoing slopes.
    // this gives us a slope to slopes map with value
    for (pos, slope_type) in &board.slopes {
        let forced_dir = slope.get(slope_type).unwrap();
        let pos = (pos.0 + forced_dir.0, pos.1 + forced_dir.1);
        
    }


    // DFS (lets hope this works :)
    fn find_longest_path(current_path: &Vec<(i32, i32)>, board: &Board) {
       // we can reuse old longest paths if the current path does not contain any of the nodes
       // found in longest 
    }

    println!("{board:?}");
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../../assets/test.txt").trim();
        assert_eq!(part1(input), "19114");
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
