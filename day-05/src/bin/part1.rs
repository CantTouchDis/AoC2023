#![cfg_attr(feature = "unstable", feature(test))]

#[cfg(not(test))]
fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(part1(input));
}

fn part1(input: &str) -> String {
    let mut lines = input.lines();
    let seeds = lines.next().unwrap().split_at(6).1.split_ascii_whitespace().map(|s| s.parse::<u64>().unwrap());
    // skip empty line
    lines.next();
    let mut previous : Vec<u64> = seeds.collect();
    let mut current : Vec<u64> = previous.clone();
    for l in lines {
        if l.is_empty() {

        }
        else if !l.chars().nth(0).unwrap().is_digit(10) {
            // start a new map
            previous = current.clone();
        }
        else {
            let m = l.split_ascii_whitespace().map(|a| a.parse::<u64>().unwrap() ).collect::<Vec<u64>>();
            current = previous.iter().zip(current.iter()).map(move |(p, c)| {
                if *c == *p && m[1] <= *p && *p < (m[1] + m[2]) {
                    m[0] + *p - m[1]
                }
                else
                {
                    *c
                }
            }).collect();
        }
    }
    current.iter().reduce(|a, b| std::cmp::min(a, b)).unwrap().to_string()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../../assets/test.txt").trim();
        assert_eq!(part1(input), "35");
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
