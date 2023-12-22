#![cfg_attr(feature = "unstable", feature(test))]

use std::collections::HashSet;

#[cfg(not(test))]
fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(part2(input));
}

fn destroy_bricks_simple(support : &HashSet<(usize, usize)>, max_brick_id : usize) -> usize {
    // find next potential brick to destroy
    let mut sum = 0;
    for i in 1..(max_brick_id) {
        // check if brick can be destroyed without letting rocks fall (direct support)
        let can_be_destroyed = support.iter().filter_map(|(a, b)| {
            if *a == i {
                Some(b)
            }
            else {
                None
            }
        }).all(|b| {
            // check if b has other supporters which were not destroyed
            support.iter().filter(|(s, a)| *a == *b && *s != i).count() > 0
        });
        if !can_be_destroyed {
            // this brick will cause at least one brick to fall.
            println!("{:?}", support.iter().filter(|(a, _b)| *a == i).map(|(_, b)| {
                support.iter().filter(|(_a, b2)| b == b2).collect::<Vec<_>>()
            }).collect::<Vec<_>>());
            // how many would fall?
            let mut falling : HashSet<usize> = HashSet::new();
            let mut lose_bricks : HashSet<usize> = HashSet::from([i]);
            while !lose_bricks.is_empty() {
                lose_bricks = support.iter().filter_map(|(a, b)|
                    if lose_bricks.contains(a) {
                        Some(*b)
                    }
                    else
                    {
                        None
                    })
                .filter(|b| {
                    // check if there are bricks still supporting the ones that became lose
                    support.iter().filter(|(a, b2)| b2 == b && (*a != i && !falling.contains(&a))).count() == 0
                })
                .collect();
                falling.extend(&lose_bricks);
            }
            println!("Removing {i} would cause {} bricks to fall.", falling.len());
            sum += falling.len();
        }
    }
    sum
}

fn part2(input: &str) -> String {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut bricks = input.lines().map(|l| {
        let (start, end) = l.split_once("~").unwrap();
        let start_coord = start.split_terminator(',').map(|c| c.parse::<u32>().unwrap()).collect::<Vec<_>>();
        let end_coord = end.split_terminator(',').map(|c| c.parse::<u32>().unwrap()).collect::<Vec<_>>();
        assert!(start_coord[0] <= end_coord[0]);
        assert!(start_coord[1] <= end_coord[1]);
        assert!(start_coord[2] <= end_coord[2]);
        max_x = std::cmp::max(max_x, end_coord[0]);
        max_y = std::cmp::max(max_y, end_coord[1]);
        ((start_coord[0], start_coord[1], start_coord[2]),
        (end_coord[0], end_coord[1], end_coord[2]))
    }).collect::<Vec<_>>();

    // sort the bricks by start coord z (just to be safe)
    bricks.sort_by(|((_, _, z1), _), ((_, _, z2), _)| z1.partial_cmp(z2).unwrap());

    let mut board = vec![vec![0_u32;max_x as usize + 1]; max_y as usize + 1];
    let mut supporter = vec![vec![0_usize;max_x as usize + 1]; max_y as usize + 1];
    // k supports v
    let mut support : HashSet<(usize, usize)> = HashSet::new();
    bricks.iter().enumerate().for_each(|(id, b)| {
        let mut max_z = 0;
        let brick_id = id + 1;
        for y in b.0.1..(b.1.1+1) {
            for x in b.0.0..(b.1.0+1) {
                max_z = std::cmp::max(max_z, board[y as usize][x as usize]);
            }
        }
        for y in b.0.1..(b.1.1+1) {
            for x in b.0.0..(b.1.0+1) {
                if board[y as usize][x as usize] == max_z {
                    support.insert((supporter[y as usize][x as usize], brick_id));
                }
                supporter[y as usize][x as usize] = brick_id; 
                board[y as usize][x as usize] = max_z + b.1.2 - b.0.2 + 1;
            }
        }
    });
    destroy_bricks_simple(&support, bricks.len()).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../../assets/test.txt").trim();
        assert_eq!(part2(input), "7");
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
