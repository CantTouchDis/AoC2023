#![cfg_attr(feature = "unstable", feature(test))]

use std::collections::HashSet;

#[cfg(not(test))]
fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(part1(input));
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

fn part1(input: &str) -> String {
    let lines = input.lines();

    let mut pattern = vec![];
    let mut sum = 0;
    lines.for_each(|l| {
        // new pattern at new lines
        if l.is_empty() {
            let column_reflections = reflection_points(&pattern);
            let pattern_t = transpose2(pattern.clone());
            let row_reflections = reflection_points(&pattern_t);
            sum += column_reflections.iter().map(|c| c).sum::<usize>();
            sum += row_reflections.iter().map(|c| c * 100).sum::<usize>();
            pattern.clear();
        }
        else
        {
            pattern.push(l.chars().collect::<Vec<_>>());
        }
    });
    let column_reflections = reflection_points(&pattern);
    let pattern_t = transpose2(pattern.clone());
    let row_reflections = reflection_points(&pattern_t);

    sum += column_reflections.iter().map(|c| c).sum::<usize>();
    sum += row_reflections.iter().map(|c| c * 100).sum::<usize>();

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../../assets/test.txt").trim();
        assert_eq!(part1(input), "405");
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
