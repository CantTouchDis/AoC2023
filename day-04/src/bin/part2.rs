#![feature(test)]
extern crate test;
use std::collections::HashSet;

fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(part2(input));
}

fn part2(input: &str) -> String {
    let lines = input.lines();
    let mut factors : Vec<u64> = vec![];
    let result = lines.map(|line| {
        let mut res :u64 = 0;
        if let Some((_, second)) = line.split_once(':') {
            if let Some((winning, mine)) = second.split_once('|') {
                let w_n = winning.split_whitespace().map(|v| v.trim().parse::<u32>().unwrap()).collect::<HashSet<_>>();
                if factors.len() < w_n.len() { factors.resize(w_n.len(), 0); }
                let m_n = mine.split_whitespace().map(|v| v.trim().parse::<u32>().unwrap()).collect::<HashSet<_>>();
                let matching = w_n.intersection(&m_n).count() as u64;
                // get the number of copies of my card
                let num_cards = factors[0] + 1;
                factors[0] = 0;
                factors.rotate_left(1);
                factors.iter_mut().enumerate().for_each(|(i, c)| {
                    if i < (matching as usize) {
                        *c += num_cards;
                    }
                });
                res = num_cards;
            }
        }
        res
    }).sum::<u64>();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    #[test]
    fn test_input() {
        let input = include_str!("../../assets/test2.txt").trim();
        assert_eq!(part2(input), "30");
    }
    #[bench]
    fn bench_main(b: &mut Bencher) {
        b.iter(|| {
            let input = test::black_box(include_str!("../../assets/input.txt").trim());
            let _ = part2(input);
        });
    }
}
