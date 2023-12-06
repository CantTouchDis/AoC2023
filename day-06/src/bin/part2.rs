#![cfg_attr(feature = "unstable", feature(test))]

#[cfg(not(test))]
fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(part2(input));
}

fn part2(input: &str) -> String {
    let mut lines = input.lines();
    let time = lines.next().unwrap().split_ascii_whitespace().skip(1).fold("".to_string(), |acc, v| {
        acc + v
    }).parse::<u64>().unwrap();
    let distance = lines.next().unwrap().split_ascii_whitespace().skip(1).fold("".to_string(), |acc, v| {
        acc + v
    }).parse::<u64>().unwrap();

    let ways_to_win = (1..(time-1)).filter(|p| {
       p * (time - p) > distance
    }).count() as i64;

    ways_to_win.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../../assets/test.txt").trim();
        assert_eq!(part2(input), "71503");
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
