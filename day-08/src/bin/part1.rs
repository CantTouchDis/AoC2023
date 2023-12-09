#![cfg_attr(feature = "unstable", feature(test))]

use std::collections::HashMap;

#[cfg(not(test))]
fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(part1(input));
}

fn part1(input: &str) -> String {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().map(|c| if c == 'L' { 0 } else { 1 });
    lines.next();

    let node_map = lines.map(|l| {
        (&l[0..3],
        [&l[7..10], &l[12..15]])
    }).collect::<HashMap<_, _>>();

    let mut current = "AAA";
    let mut steps = 0;
    for i in instructions.cycle() {
        current = node_map.get(current).unwrap()[i];
        steps += 1;
        if current == "ZZZ" {
            break;
        }

    }

    steps.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../../assets/test.txt").trim();
        assert_eq!(part1(input), "2");
        let input = include_str!("../../assets/test2.txt").trim();
        assert_eq!(part1(input), "6");
    }
}

#[cfg(all(feature = "unstable", test))]
mod bench {
    use super::*;
    extern crate test;
    use self::test::Bencher;

    #[bench]
    fn bench_main(b: &mut Bencher) {
        b.iter(|| {
            let input = test::black_box(include_str!("../../assets/input.txt").trim());
            let _ = part1(input);
        });
    }
}
