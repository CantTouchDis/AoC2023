#![cfg_attr(feature = "unstable", feature(test))]

use std::collections::HashSet;

#[cfg(not(test))]
fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(part2(input));
}
fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn reflection_points(v: &Vec<Vec<char>>) -> HashSet<usize> {
    let line_length = v[0].len();
    let mut reflections = (1..(line_length)).collect::<HashSet<_>>();

    v.iter().for_each(|l| {
        reflections.retain(|start| {
            // checks if this could be a reflection point
            l[0..*start].iter().rev()
                .zip(l[*start..].iter())
                .all(|(a, b)| a == b)
        });
    });
    reflections
}

fn part2(input: &str) -> String {
    let lines = input.lines();

    let mut pattern : Vec<Vec<_>> = vec![];
    let mut sum = 0;

    lines.for_each(|l| {
        // new pattern at new lines
        if l.is_empty() {
            let row_length = pattern[0].len();
            let old_column_reflections = reflection_points(&pattern);
            let pattern_t = transpose2(pattern.clone());
            let old_row_reflections = reflection_points(&pattern_t);
            for y in 0..pattern.len() {
                for x in 0..row_length {
                    // flip entry
                    if pattern[y][x] == '#' {
                        pattern[y][x] = '.';
                    }
                    else
                    {
                        pattern[y][x] = '#';
                    }
                    let column_reflections = reflection_points(&pattern).difference(&old_column_reflections).cloned().collect::<Vec<usize>>();
                    let pattern_t = transpose2(pattern.clone());
                    let row_reflections = reflection_points(&pattern_t).difference(&old_row_reflections).cloned().collect::<Vec<usize>>();
                    if !column_reflections.is_empty() || !row_reflections.is_empty() {
//                        println!("{row_reflections:?} {column_reflections:?} at {x}, {y}");
                        sum += column_reflections.iter().map(|c| c).sum::<usize>();
                        sum += row_reflections.iter().map(|c| c * 100).sum::<usize>();
                        pattern.clear();
                        return;
                    }
                    // flip back
                    if pattern[y][x] == '#' {
                        pattern[y][x] = '.';
                    }
                    else
                    {
                        pattern[y][x] = '#';
                    }
                }
            }
        }
        else
        {
            pattern.push(l.chars().collect::<Vec<_>>());
        }
    });
    let row_length = pattern[0].len();
    let old_column_reflections = reflection_points(&pattern);
    let pattern_t = transpose2(pattern.clone());
    let old_row_reflections = reflection_points(&pattern_t);
    for y in 0..pattern.len() {
        for x in 0..row_length {
            // flip entry
            if pattern[y][x] == '#' {
                pattern[y][x] = '.';
            }
            else
            {
                pattern[y][x] = '#';
            }
            let column_reflections = reflection_points(&pattern).difference(&old_column_reflections).cloned().collect::<Vec<usize>>();
            let pattern_t = transpose2(pattern.clone());
            let row_reflections = reflection_points(&pattern_t).difference(&old_row_reflections).cloned().collect::<Vec<usize>>();
            if !column_reflections.is_empty() || !row_reflections.is_empty() {
//                println!("{row_reflections:?} {column_reflections:?} at {x}, {y}");
                sum += column_reflections.iter().map(|c| c).sum::<usize>();
                sum += row_reflections.iter().map(|c| c * 100).sum::<usize>();
                return sum.to_string();
            }
            // flip back
            if pattern[y][x] == '#' {
                pattern[y][x] = '.';
            }
            else
            {
                pattern[y][x] = '#';
            }
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../../assets/test.txt").trim();
        assert_eq!(part2(input), "400");
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
