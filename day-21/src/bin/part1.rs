#![cfg_attr(feature = "unstable", feature(test))]

use std::collections::HashSet;

#[cfg(not(test))]
fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(part1(input, 64));
}


fn part1(input: &str, steps: usize) -> String {
    let blocked_map = input.lines().enumerate().flat_map(|(y, l)| {
        l.chars().enumerate().filter_map(move |(x, c)| {
            if c == '#' {
                Some((y as i32,x as i32))
            }
            else {
                None
            }
        })
    }).collect::<HashSet<(i32, i32)>>();
    let start_pos = input.lines().enumerate().find_map(|(y, l)| {
        if let Some(x) = l.chars().enumerate().find_map(|(x, c)| {
            if c == 'S' {
                Some(x)
            }
            else {
                None
            }
        }) {
            Some((y as i32, x as i32))
        }
        else {
            None
        }
    }).unwrap();
    let board_dim = [input.lines().count() as i32, input.lines().next().unwrap().len() as i32];


    let mut wave_front = HashSet::from([start_pos]);
    let mut spaces : [HashSet<(i32, i32)>; 2] = [HashSet::new(), HashSet::new()];

    for i in 1..(steps + 1) {
        wave_front = wave_front.iter().flat_map(|&(y, x)| {
            [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)].iter().filter_map(|&(y, x)| {
                if (y < 0 || x < 0 || y >= board_dim[0] || x >= board_dim[1]) {
                    None
                }
                else if blocked_map.contains(&(y, x)) {
                    None
                }
                else {
                    Some((y, x))
                }
            }).collect::<Vec<_>>()
        }).collect::<HashSet<_>>();
        spaces[i % 2].extend(&wave_front);
    }
    let result = spaces[steps % 2].len();


    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../../assets/test.txt").trim();
        assert_eq!(part1(input, 6), "16");
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
            let _ = part1(input, 64);
        });
    }
}
