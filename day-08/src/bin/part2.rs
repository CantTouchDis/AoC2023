#![cfg_attr(feature = "unstable", feature(test))]

use std::collections::{HashSet, HashMap};

#[cfg(not(test))]
fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(part2(input));
}

// This function searches for a cycle in the data it panics if the goal is not
// after exactly the same step_count, cause this would break the solution below
fn find_cycle(start: &str, instructions: &Vec<i32>, directions: &HashMap<&str, [&str; 2]>) -> Option<u64> {
    let mut visited_at = HashMap::<(&str, usize), u64>::new();
    let mut step_count : u64 = 0;
    let mut current : &str = start;
    let mut goal_at_steps = 0;
    for (cycle_id, &i) in instructions.iter().enumerate().cycle() {
        let old_value = visited_at.insert((current, cycle_id), step_count);
        if let Some(val) = old_value {
            // println!("Found a cycle from {val} to {step_count} steps: {}", step_count - val);
            if step_count - val != goal_at_steps {
                return None;
            }
            else {
                return Some(goal_at_steps);
            }
        }
        if current.chars().nth(2).unwrap() == 'Z' {
            goal_at_steps = step_count;
            println!("There is a solution after {step_count} steps");
        }
        current = directions.get(current).unwrap()[i as usize];
        step_count += 1;
    }
    None
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn part2(input: &str) -> String {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().map(|c| if c == 'L' { 0 } else { 1 }).collect::<Vec<_>>();
    lines.next();

    let mut currents = HashSet::new();

    let node_map = lines.map(|l| {
        if l.chars().nth(2).unwrap() == 'A' {
            currents.insert(&l[0..3]);
        }
        (&l[0..3],
        [&l[7..10], &l[12..15]])
    }).collect::<HashMap<_, _>>();

    let result = currents.iter().filter_map(|c| find_cycle(c, &instructions, &node_map)).fold(1, |acc, c| {
       lcm(acc, c as usize)
    });

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../../assets/test3.txt").trim();
        assert_eq!(part2(input), "6");
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
