#![cfg_attr(feature = "unstable", feature(test))]

#[cfg(not(test))]
fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(part1(input));
}


fn advent_hash(line: &str) -> i32 {
    line.chars().map(|c| c as i32).fold(0, |acc, v| {
        (acc + v) * 17 % 256
    })
}

fn part1(input: &str) -> String {
    let input= input.split_terminator(',');

    let result = input.map(|l| advent_hash(l)).sum::<i32>();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        assert_eq!(advent_hash("HASH"), 52);
        let input = include_str!("../../assets/test.txt").trim();
        assert_eq!(part1(input), "1320");
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
