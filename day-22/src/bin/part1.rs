#![cfg_attr(feature = "unstable", feature(test))]

use std::collections::HashSet;

#[cfg(not(test))]
fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(part1(input));
}

fn destroy_bricks_simple(support : &HashSet<(usize, usize)>, max_brick_id : usize) -> usize {
    // find next potential brick to destroy
    let mut sum = 0;
    for i in 1..(max_brick_id+1) {
        // check if brick can be destroyed
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
        if can_be_destroyed {
            println!("{i} will be destroyed!");
            sum += 1;
        }
    }
    sum
}

fn part1(input: &str) -> String {
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
        //println!("Brick {b:?}");
        //for y in 0..(max_y+1) {
        //    let mut line = String::new();
        //    for x in 0..(max_x+1) {
        //        line.push_str(&format!(" {:03} ", board[y as usize][x as usize]));
        //    }
        //    println!("{line}");
        //}
        //println!("");
    });

    // start to fill a x * y big square
    //
    //let mut destroyed_bricks = HashSet::new();
    // destroy top down
    //let destroyed_bricks = destroy_bricks(&support, &HashSet::new(), bricks.len()+1);
    destroy_bricks_simple(&support, bricks.len()).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../../assets/test.txt").trim();
        assert_eq!(part1(input), "5");
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
