#![cfg_attr(feature = "unstable", feature(test))]
use std::collections::HashSet;

#[cfg(not(test))]
fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(part1(input));
}

fn part1(input: &str) -> String {
    let lines = input.lines();
    let result = lines.map(|line| {
        let mut res :u32 = 0;
        if let Some((_, second)) = line.split_once(':') {
            if let Some((winning, mine)) = second.split_once('|') {
                let w_n = winning.split_whitespace().map(|v| v.trim().parse::<u32>().unwrap()).collect::<HashSet<_>>();
                let m_n = mine.split_whitespace().map(|v| v.trim().parse::<u32>().unwrap()).collect::<HashSet<_>>();
                let c = w_n.intersection(&m_n).count() as u32;
                if c != 0 {
                    res = 2_u32.pow(c - 1);
                }
            }
        }
        res
    }).sum::<u32>();
    result.to_string()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../../assets/test.txt").trim();
        assert_eq!(part1(input), "13");
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
