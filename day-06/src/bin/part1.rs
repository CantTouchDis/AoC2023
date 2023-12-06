#![cfg_attr(feature = "unstable", feature(test))]

#[cfg(not(test))]
fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(part1(input));
}

fn part1(input: &str) -> String {
    let mut lines = input.lines();
    let times = lines.next().unwrap().split_ascii_whitespace().filter_map(|t| t.parse::<i32>().ok());
    let distances = lines.next().unwrap().split_ascii_whitespace().filter_map(|t| t.parse::<i32>().ok());

    let ways_to_win = times.zip(distances).map(|(a, b)| {
        (1..(a-1)).filter(|p| {
           p * (a - p) > b
        }).count() as i32
    }).product::<i32>();

    ways_to_win.to_string()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../../assets/test.txt").trim();
        assert_eq!(part1(input), "288");
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
