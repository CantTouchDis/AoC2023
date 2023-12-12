#![cfg_attr(feature = "unstable", feature(test))]

#[cfg(not(test))]
fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(part1(input));
}

fn part1(input: &str) -> String {
    let lines = input.lines();
    let gear_groups = lines.map(|l| {
        let mut it = l.split_ascii_whitespace();
        let gears = it.next().unwrap().replace('.', " ");
        let groups = it.next().unwrap().split(',').map(|c| c.parse::<i32>().unwrap()).collect::<Vec<_>>();
        (gears, groups)
    }).collect::<Vec<_>>();

    let result = gear_groups.iter().map(|(gears, groups)| {
        let num_unknown = gears.chars().filter(|c| *c == '?').count() as u32;
        let mut num_working = 0;
        for i in 0..(2_usize.pow(num_unknown)) {
            // modify using the arangement
            let mut idx = 1;
            let arangement = gears.chars().map(|c| {
                if c == '?' {
                    let v = idx;
                    idx *= 2;
                    if i & v == v {
                        '#'
                    }
                    else
                    {
                        ' '
                    }
                }
                else
                {
                    c
                }
            }).collect::<String>();
            let arangement = arangement.split_ascii_whitespace().map(|l| l.len()).collect::<Vec<_>>();
            if groups.len() == arangement.len() && groups.iter().zip(arangement).all(|(a, b)| *a == b as i32) {
                num_working += 1;
            }
        }
        num_working
    }).sum::<i32>();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../../assets/test.txt").trim();
        assert_eq!(part1(input), "21");
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
