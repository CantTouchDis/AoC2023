#![cfg_attr(feature = "unstable", feature(test))]


#[cfg(not(test))]
fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(part1(input));
}

fn part1(input: &str) -> String {
    let lines = input.lines();
    let result =lines.map(|l| {
        let mut values : Vec<Vec<i32>> = vec![];
        values.push(l.split_whitespace().map(|v| v.parse::<i32>().unwrap()).collect::<Vec<_>>());
        while !values.last().unwrap().iter().all(|&v| v == 0) {
            values.push(values.last().unwrap().windows(2)
                .map(|w| {
                    w[1] - w[0]
            }).collect::<Vec<_>>());
        }
        values.iter().rev().fold(0, |acc, v| {
            acc + v.last().unwrap()
        })
    }).sum::<i32>();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../../assets/test.txt").trim();
        assert_eq!(part1(input), "114");
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
