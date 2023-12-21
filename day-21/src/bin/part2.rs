#![cfg_attr(feature = "unstable", feature(test))]

use std::collections::HashSet;

#[cfg(not(test))]
fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    //dbg!(part2(input, 100));
    dbg!(part2(input, 26501365));
}

fn part2(input: &str, steps: usize) -> String {
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
    println!("{board_dim:?}");


    let mut wave_front = HashSet::from([start_pos]);
    let mut result_spaces : HashSet<(i32, i32)> = HashSet::new();



    if steps <= 100 {
        for i in 1..(steps + 1) {
            wave_front = wave_front.iter().flat_map(|&(y, x)| {
                [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)].iter().filter_map(|&(y, x)| {
                    if blocked_map.contains(&(y.rem_euclid(board_dim[0]), x.rem_euclid(board_dim[1]))) {
                        None
                    }
                    else {
                        Some((y, x))
                    }
                }).collect::<Vec<_>>()
            }).collect::<HashSet<_>>();
            if i % 2 == 0 {
                result_spaces.extend(&wave_front);
            }
        }
        for y in -80..80_i32 {
            let mut line = String::new();
            for x in -80..80_i32 {
                if blocked_map.contains(&(y.rem_euclid(board_dim[0]), x.rem_euclid(board_dim[1]))) {
                    line.push('#');
                }
                else if result_spaces.contains(&(y, x)) {
                    line.push('O');
                }
                else {
                    line.push('.')
                }
            }
            println!("{line}");
        }
        // return result_spaces.len().to_string();
    }

    // for every entry point on the edge compute how long it takes to fill the whole block
    let corners : [(i32, i32); 4] = [(0, 0), (0, board_dim[1] - 1), (board_dim[0] - 1, 0), (board_dim[0] - 1, board_dim[1] - 1)];
    let corner_spaces : Vec<(_, i32)> = corners.iter().map(|c| {
        let mut spaces : [HashSet<(i32, i32)>; 2] = [HashSet::new(), HashSet::new()];
        let mut wave_front : HashSet<(i32, i32)> = HashSet::from([*c]);
        for i in 1..300 {
            wave_front = wave_front.iter().flat_map(|&(y, x)| {
                [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)].iter().filter_map(|&(y, x)| {
                    if y < 0 || x < 0 || y >= board_dim[0] || x >= board_dim[1] {
                        None
                    }
                    else if blocked_map.contains(&(y, x)) {
                        None
                    }
                    else if spaces[i % 2].contains(&(y, x)) {
                        None
                    }
                    else {
                        Some((y, x))
                    }
                }).collect::<Vec<_>>()
            }).collect::<HashSet<_>>();
            if wave_front.is_empty() {
                return ((spaces[0].len(), spaces[1].len()), i as i32);
            }
            spaces[i % 2].extend(&wave_front);
        }
        panic!();
        }).collect::<Vec<_>>();
    let first_corner_encouter_steps = if cfg!(test) {
        [10, 10, 10, 14]
    }
    else {
        [130; 4]
    };
    let min_encounter = first_corner_encouter_steps.iter().min().unwrap();


    // compute the fill
    let count_rem = first_corner_encouter_steps.iter().map(|c| steps - c).map(|remaining_steps|
        (remaining_steps / board_dim[0] as usize, remaining_steps % board_dim[0] as usize, 
         remaining_steps / board_dim[1] as usize, remaining_steps % board_dim[1] as usize))
        .collect::<Vec<_>>();
    let mut completed = count_rem.iter().map(|(a, _, b, _)| {
        assert_eq!(a, b);
        // gauss
        // this is just to be conservative. it does not hurt to do a bit of extra computation
        let a = a - 1;
        (a * (a + 1)) / 2
    }).sum::<usize>();
    completed += count_rem.iter().map(|(a, _, b, _)| {
        assert_eq!(a, b);
        std::cmp::max(0, a - 2)
    }).sum::<usize>();

    // this is excluding the start...
    let sum_odd_event = corner_spaces[0].0.0 + corner_spaces[0].0.1;
    let count_completed = sum_odd_event * (completed / 2);


    // for each not accounted for board at the edge we compute:
    let edge_cases : Vec<(usize, usize)> = corners.iter().zip(count_rem.iter().rev()).map(|(c, cr)| {
        let mut spaces : [HashSet<(i32, i32)>; 2] = [HashSet::new(), HashSet::new()];
        let mut wave_front : HashSet<(i32, i32)> = HashSet::from([*c]);
        let mut a : usize = 0;
        assert_eq!(cr.1, cr.3);
        assert_eq!(board_dim[0], board_dim[1]);
        let max_steps = cr.1 + board_dim[0] as usize;

        for i in 1..(max_steps - 1) {
            wave_front = wave_front.iter().flat_map(|&(y, x)| {
                [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)].iter().filter_map(|&(y, x)| {
                    if y < 0 || x < 0 || y >= board_dim[0] || x >= board_dim[1] {
                        None
                    }
                    else if blocked_map.contains(&(y, x)) {
                        None
                    }
                    else if spaces[i % 2].contains(&(y, x)) {
                        None
                    }
                    else {
                        Some((y, x))
                    }
                }).collect::<Vec<_>>()
            }).collect::<HashSet<_>>();
            spaces[i % 2].extend(&wave_front);
            if i + 1 == cr.1 {
                println!("spaces for {c:?}: {:?}", spaces[i%2]);
                a = spaces[(i - 1) % 2].len();
            }
        }
        (a, spaces[max_steps % 2].len())
        }).collect::<Vec<_>>();

    let count_edge_cases = edge_cases.iter().zip(count_rem.iter()).map(|(a, rem)| {
        assert_eq!(rem.0, rem.2);
        rem.0 * a.0 + rem.0 * a.1 + a.1
    }).sum::<usize>();

    let hor_vert_spaces : Vec<(_, i32)> = [
        ((corners[0], first_corner_encouter_steps[0]), (corners[1], first_corner_encouter_steps[1])),
        ((corners[0], first_corner_encouter_steps[0]), (corners[2], first_corner_encouter_steps[2])),
        ((corners[1], first_corner_encouter_steps[1]), (corners[3], first_corner_encouter_steps[3])),
        ((corners[2], first_corner_encouter_steps[2]), (corners[3], first_corner_encouter_steps[3])),
    ]
        .iter().zip(count_rem.iter().rev()).map(|(c, cr)| {
        let mut spaces : [HashSet<(i32, i32)>; 2] = [HashSet::new(), HashSet::new()];
        let mut wave_front : HashSet<(i32, i32)> = HashSet::new();
        let mut a : usize = 0;
        let add_to_wavefront = |s : usize, wave : &mut HashSet<(i32, i32)>| {
            if c.0.1 == s + min_encounter {
                wave.insert(c.0.0);
            }
            if c.1.1 == s + min_encounter {
                wave.insert(c.1.0);
            }
        };
        add_to_wavefront(0, &mut wave_front);
        assert_eq!(cr.1, cr.3);
        assert_eq!(board_dim[0], board_dim[1]);
        let max_steps = cr.1 + board_dim[0] as usize;
        for i in 1..(max_steps - 1) {
            wave_front = wave_front.iter().flat_map(|&(y, x)| {
                [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)].iter().filter_map(|&(y, x)| {
                    if y < 0 || x < 0 || y >= board_dim[0] || x >= board_dim[1] {
                        None
                    }
                    else if blocked_map.contains(&(y, x)) {
                        None
                    }
                    else if spaces[i % 2].contains(&(y, x)) {
                        None
                    }
                    else {
                        Some((y, x))
                    }
                }).collect::<Vec<_>>()
            }).collect::<HashSet<_>>();
            add_to_wavefront(i, &mut wave_front);
            spaces[i % 2].extend(&wave_front);
        }
        (0, 0)
        }).collect::<Vec<_>>();
    println!("{completed} {count_completed:?} {count_edge_cases:?} {}", count_completed + count_edge_cases);


    // with this we can compute the radius of filled gardens:

    
    println!("{count_rem:?}");

    println!("{hor_vert_spaces:?}");


    // the remaining squares need to be computed
    // observations from printing the map is that we got a repeating pattern every other map
    // maps all have odd dimensions
    // // after a long time we can assume that the waves traveling through the "tunnels" are the
    // fastest therefore we can just precompute the fillings for the 4 corner entries
    // to arrive at the solution we need to find the first 4 corner encounter values

    for y in -200..200_i32 {
        let mut line = String::new();
        for x in -200..200_i32 {
            if blocked_map.contains(&(y.rem_euclid(board_dim[0]), x.rem_euclid(board_dim[1]))) {
                line.push('#');
            }
            else if result_spaces.contains(&(y, x)) {
                line.push('O');
            }
            else {
                line.push('.')
            }
        }
        println!("{line}");
    }
    //println!("{result_spaces:?}");
    let result = result_spaces.len();


    result.to_string()
}

struct Board {
    width: usize,
    height: usize,
    blocked: HashSet<(i32, i32)>,
}


fn step_n(board: &Board, wave_front: HashSet<(i32, i32)>, steps: usize) -> HashSet<(i32, i32)> {
    let mut wave_front = wave_front;
    for _i in 0..steps {
        wave_front = wave_front.iter().flat_map(|&(y, x)| {
            [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)].iter().filter_map(|&(y, x)| {
                if y < 0 || x < 0 || y >= board.height as i32 || x >= board.width as i32 {
                    None
                }
                else if board.blocked.contains(&(y, x)) {
                    None
                }
                else {
                    Some((y, x))
                }
            }).collect::<Vec<_>>()
        }).collect::<HashSet<_>>();
    }
    wave_front
}

fn print_state(board : &Board, wave_front : &HashSet<(i32, i32)>) {
    for y in 0..board.height {
        let mut line = String::new();
        for x in 0..board.width {
            if board.blocked.contains(&(y as i32, x as i32)) {
                line.push('#');
            }
            else if wave_front.contains(&(y as i32, x as i32)) {
                line.push('O');
            }
            else {
                line.push('.')
            }
        }
        println!("{line}");
    }
    println!("");
}

fn part2_2(input: &str, steps: usize) -> String {
    let board = {
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
        Board { width: input.lines().next().unwrap().len(), height: input.lines().count(), blocked: blocked_map }
    };
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

    // find the first time the corners are contained in the wavefront
    let corners : [(i32, i32); 4] = [
        (0, 0),
        (0, board.width as i32 - 1),
        (board.height as i32 - 1, 0),
        (board.height as i32 - 1, board.width as i32 - 1)
    ];
    let mut found_after : [usize; 4] = [0; 4];
    let mut i : usize = 0;
    let mut wave_front = HashSet::from([start_pos]);
    while found_after.iter().any(|&c| c == 0) {
        // new wave_front (on cropped board)
        i += 1;
        wave_front = step_n(&board, wave_front, 1);
        for (corner, found) in corners.iter().zip(found_after.iter_mut()) {
            if *found == 0 && wave_front.contains(corner) {
                *found = i;
            }
        }
    }
    let mut previous = if i % 2 == 0 {
        [wave_front.clone(), HashSet::new()]
    }
    else {
        [HashSet::new(), wave_front.clone()]
    };
    i += 1;
    wave_front = step_n(&board, wave_front, 1);
    previous[i%2] = wave_front.clone();
    loop {
        i += 1;
        wave_front = step_n(&board, wave_front, 1);
        if wave_front == previous[i%2] {
            break;
        }
        previous[i%2].extend(&wave_front);
    }
    let first_filled_after = i;
    let parity_counts = [previous[0].len(), previous[1].len()];

    // compute it manually
    if steps < first_filled_after {
        return "".to_string();
    }
    // the general case is computed here:
    // first find in the horizontal direction how long it takes to fill the board
    // after encountering the corners as computed above
    // NOTE: the test case does not arrive at all the corners at the same time, but the
    // real data does, how curious

    // completed box count starts with the center one (even parity)
    let mut completed_boxes = [1, 0];
    // add counts directly to the sides of the initial box
    // this can be computed by checking how long it takes to fill a box from each side
    let edge_fill_indices : [(usize, usize, usize, usize); 4] = [
        (0, 1, 2, 3), // UP
        (1, 3, 0, 2), // RIGHT
        (2, 3, 0, 1), // DOWN
        (0, 2, 1, 3), // LEFT
    ];
    let time_to_fill = edge_fill_indices.iter().map(|(c1, c2, c3, c4)| {
        let offset : usize = std::cmp::min(found_after[*c1], found_after[*c2]);
        let mut wave_front = HashSet::new();
        let mut i : usize = offset;
        let mut previous : [HashSet<(i32, i32)>; 2] = [HashSet::new(), HashSet::new()];
        loop {
            if i == found_after[*c1] {
                wave_front.insert(corners[*c3]);
            }
            if i == found_after[*c2] {
                wave_front.insert(corners[*c4]);
            }
            i += 1;
            wave_front = step_n(&board, wave_front, 1);
            if wave_front == previous[i % 2] {
                break;
            }
            previous[i % 2].extend(&wave_front);
        }
        i - offset
    }).collect::<Vec<usize>>();
    println!("{time_to_fill:?}");

    // assume width = height
    assert_eq!(board.width, board.height);

    let filled_box_count = found_after.map(|a| (steps - a) / board.height).map(|count| count - 1);
    let remaining_steps_per_dir = filled_box_count.map(|rem| steps - rem * board.height);
    let filled_box_count = filled_box_count.iter().zip(remaining_steps_per_dir.iter().zip(time_to_fill.iter()).map(|(rem, fill)| {
        if rem > fill {
            1
        }
        else {
            0
        }
    })).map(|(c, a)| c + a).collect::<Vec<usize>>();
    for c in &filled_box_count {
        completed_boxes[0] += (c - 1) / 2;
        completed_boxes[1] += (c) / 2;
    }
    let remaining_steps_per_dir = filled_box_count.iter().map(|rem| steps - rem * board.height).collect::<Vec<_>>();
    println!("{remaining_steps_per_dir:?}");

    println!("{completed_boxes:?}");
    // compute how many spaces are filled with the remaining steps per direction:
    let mut remaining_visited = edge_fill_indices.iter().zip(remaining_steps_per_dir.iter()).map(|((c1, c2, c3, c4), rem)| {
        let offset : usize = std::cmp::min(found_after[*c1], found_after[*c2]);
        let mut wave_front = HashSet::new();
        for i in 0..*rem {
            let i = i + offset;
            if i == found_after[*c1] {
                wave_front.insert(corners[*c3]);
            }
            if i == found_after[*c2] {
                wave_front.insert(corners[*c4]);
            }
            wave_front = step_n(&board, wave_front, 1);
        }
        print_state(&board, &wave_front);
        wave_front.len()
    }).sum::<usize>();

    remaining_visited += edge_fill_indices.iter().zip(remaining_steps_per_dir.iter().map(|r| r - board.height)).map(|((c1, c2, c3, c4), rem)| {
        let offset : usize = std::cmp::min(found_after[*c1], found_after[*c2]);
        let mut wave_front = HashSet::new();
        for i in 0..rem {
            let i = i + offset;
            if i == found_after[*c1] {
                wave_front.insert(corners[*c3]);
            }
            if i == found_after[*c2] {
                wave_front.insert(corners[*c4]);
            }
            wave_front = step_n(&board, wave_front, 1);
        }
        print_state(&board, &wave_front);
        wave_front.len()
    }).sum::<usize>();

    // all thats left to do is to count the diagonal remaining elements:
    let filled_diagonal_count = found_after.map(|a| (steps - a) / board.height).map(|count| count - 1);
    let mut remaining_steps_per_dir = filled_diagonal_count.map(|rem| steps - rem * board.height);
    // this can be done by adding the completely filled elements
    println!("{remaining_steps_per_dir:?}");


    let corner_fill_indices : [(usize, usize); 4]= [
        (0, 3), //TOP_LEFT
        (1, 2), // TOP_RIGHT
        (3, 0), // BOT_RIGHT
        (2, 1), // BOT_LEFT
    ];
    let mut count : usize = *filled_diagonal_count.iter().min().unwrap();
    println!("{count}, ADD {}", count);
    println!("{completed_boxes:?}");
    completed_boxes[count % 2] += 4 * (count / 2).pow(2);
    completed_boxes[(count + 1) % 2] += 4 * (count / 2) * ((count / 2) + 1);
    println!("{completed_boxes:?}");
    let mut sum = 0;
    while remaining_steps_per_dir.iter().any(|&s| s > 0) {
        sum += (count + 1) * corner_fill_indices.iter().zip(remaining_steps_per_dir.iter()).map(|((c1, c2), rem)| {
            if *rem <= found_after[*c1] + 2 {
                return 0;
            }
            let remaining_steps = *rem - found_after[*c1];
            let mut wave_front = HashSet::from([corners[*c2]]);
            wave_front = step_n(&board, wave_front, remaining_steps - 2);
            print_state(&board, &wave_front);
            wave_front.len()
        }).sum::<usize>();
        remaining_steps_per_dir.iter_mut().for_each(|c| if *c > board.height { *c = *c - board.height } else { *c = 0 } );
        count += 1;
    }
    (
        parity_counts[0] * completed_boxes[0] + 
        parity_counts[1] * completed_boxes[1] + 
        sum + remaining_visited
    ).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../../assets/test.txt").trim();
        //assert_eq!(part2(input, 6), "16");
        //assert_eq!(part2(input, 10), "50");

        //assert_eq!(part2(input, 50), "1594");
        //assert_eq!(part2_2(input, 50), "1594");
        //assert_eq!(part2(input, 100), "6536");
        assert_eq!(part2_2(input, 100), "6536");
        //assert_eq!(part2_2(input, 500), "167004");
        //assert_eq!(part2(input, 1000), "668697");
        //assert_eq!(part2(input, 5000), "16733044");
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
