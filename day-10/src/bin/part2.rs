#![cfg_attr(feature = "unstable", feature(test))]

use std::collections::{HashMap, HashSet};

#[cfg(not(test))]
fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(part2(input));
}


fn part2(input: &str) -> String {
    let mut neighbours : HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    neighbours.insert('S', vec![(-1, 0), (0, -1), (0, 1), (1, 0)]);
    neighbours.insert('|', vec![(0, -1), (0, 1)]);
    neighbours.insert('-', vec![(-1, 0), (1, 0)]);
    neighbours.insert('L', vec![(0, -1), (1, 0)]);
    neighbours.insert('J', vec![(-1, 0), (0, -1)]);
    neighbours.insert('7', vec![(-1, 0), (0, 1)]);
    neighbours.insert('F', vec![(0, 1), (1, 0)]);

    let empty : Vec<(i32, i32)> = vec![];

    let mut start : (i32, i32) = (-1, -1);

    let lines = input.lines();
    let board = lines.enumerate().map(|(y, l)| {
        l.chars().enumerate().map(|(x, c)| {
            let n = neighbours.get(&c).unwrap_or(&empty);
            if n.len() == 4 {
                start = (x as i32, y as i32);
            }
            n
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    for i in 0..4 {
        let mut move_to = neighbours.get(&'S').unwrap()[i];
        let mut next_tile = (start.0 + move_to.0, start.1 + move_to.1);
        let mut visited : HashSet<(i32, i32)> = HashSet::new();
        visited.insert(start);
        let mut area = 0;
        let mut length = 0;
        while visited.insert(next_tile) {
            // new tile!
            let neighbours = board[next_tile.1 as usize][next_tile.0 as usize];
            let neighbours = neighbours.iter().filter(|(x, y)| {
                !(move_to.0 == -x && move_to.1 == -y)
            }).collect::<Vec<_>>();
            if neighbours.len() != 1
            {
                break;
            }
            move_to = *neighbours[0];
            area += (next_tile.1 + next_tile.1 + move_to.1) * move_to.0;
            next_tile = (next_tile.0 + move_to.0, next_tile.1 + move_to.1);
            length += 1;
        }
        if next_tile == start {
            // the polygon area is overcounting by half of the length of the loop
            // subtract it to find the correct solution!
            return (area / 2 - length / 2).to_string();
        }
    }
    "No Loop -> No Solution".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../../assets/test.txt").trim();
        assert_eq!(part2(input), "1");
        let input = include_str!("../../assets/test2.txt").trim();
        assert_eq!(part2(input), "10");
        let input = include_str!("../../assets/test3.txt").trim();
        assert_eq!(part2(input), "4");
    }
}

#[cfg(all(feature = "unstable", test))]
mod bench {
    use super::*;
    extern crate test;
    use self::test::Bencher;

    #[bench]
    fn bench_main(b: &mut Bencher) {
        let input = test::black_box(include_str!("../../assets/input.txt").trim());
        b.iter(move || {
            //std::thread::sleep(std::time::Duration::from_nanos(1000));
            let _ = part2(input);
        });
    }
}
