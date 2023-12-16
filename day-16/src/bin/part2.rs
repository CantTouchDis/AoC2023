#![cfg_attr(feature = "unstable", feature(test))]

use std::collections::{HashMap, HashSet};

#[cfg(not(test))]
fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(part2(input));
}

fn mirror_interactions(beam_dir: (i32, i32), mirror: char) -> Vec<(i32, i32)> {
    match mirror {
        '/' => vec![(-beam_dir.1, -beam_dir.0)],
        '\\' => vec![(beam_dir.1, beam_dir.0)],
        '|' if beam_dir.0 != 0 => vec![beam_dir],
        '|' if beam_dir.0 == 0 => vec![(beam_dir.1, 0), (-beam_dir.1, 0)],
        '-' if beam_dir.1 != 0 => vec![beam_dir],
        '-' if beam_dir.1 == 0 => vec![(0, beam_dir.0), (0, -beam_dir.0)],
        _ => todo!(),

    }
}

fn part2(input: &str) -> String {
    let mirror_map = input.lines().enumerate().flat_map(|(i, l)| {
        l.chars()
            .enumerate()
            .filter(|(_, c)| *c != '.')
            .map(move |(j, c)| {
            ((i as i32, j as i32), c)
        })
    }).collect::<HashMap<(i32, i32), char>>();

    let board_width = input.lines().nth(0).unwrap().len() as i32;
    let board_height = input.lines().count() as i32;

    let mut current_beam_fronts = //vec![((0, 0), (0, 1))];
        (0..board_height).map(|y| ((y, 0), (0, 1)))
            .chain((0..board_height).map(|y| ((y, board_width - 1), (0, -1))))
            .chain((0..board_width).map(|x| ((0, x), (1, 0))))
            .chain((0..board_width).map(|x| ((board_height - 1, x), (-1, 0))))
        .collect::<Vec<_>>();


    current_beam_fronts.iter().map(|(pos, dir)| {
        let mut current_beam_fronts = vec![(*pos, *dir)];
        let mut encountered_mirrors : HashSet<((i32, i32), (i32, i32))> = HashSet::new();
        let mut encountered : HashSet<(i32, i32)> = HashSet::new();
        encountered.insert(*pos);
        while !current_beam_fronts.is_empty() {
            current_beam_fronts = current_beam_fronts.iter().flat_map(|(pos, dir)| {
                if let Some(c) = mirror_map.get(&pos) {
                    // if we already encountered this mirror from the same direction we stop.
                    if encountered_mirrors.insert((*pos, *dir)) {
                        mirror_interactions(*dir, *c).iter().map(|new_dir| ((pos.0 + new_dir.0, pos.1 + new_dir.1), *new_dir)).collect::<Vec<_>>()
                    }
                    else
                    {
                        vec![]
                    }
                }
                else {
                    vec![((pos.0 + dir.0, pos.1 + dir.1), *dir)]
                }
            })
            .filter(|(pos, _)| {
                !(pos.0 < 0 || pos.1 < 0) && !(pos.0 >= board_height || pos.1 >= board_width)
            })
            .collect::<Vec<_>>();
            current_beam_fronts.iter().for_each(|(pos, _dir)| {
                encountered.insert(*pos);
            });
        }
        encountered.len()
    }).max().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../../assets/test.txt").trim();
        assert_eq!(part2(input), "51");
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
