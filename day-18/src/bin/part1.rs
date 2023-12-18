#![cfg_attr(feature = "unstable", feature(test))]

#[cfg(not(test))]
fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(part1(input));
}

fn part1(input: &str) -> String {
    let mut current = (0, 0);
    let mut area = 0;
    let mut perimeter = 0;
    input.lines()
        .map(|l| {
            let mut plan_line = l.split_ascii_whitespace();
            (plan_line.next().unwrap().chars().next().unwrap(),
            plan_line.next().unwrap().parse::<i32>().unwrap())
        })
    .for_each(|(dir, steps)| {
        let tmp = current;
        match dir {
            'R' => current.0 += steps,
            'L' => current.0 -= steps,
            'U' => current.1 -= steps,
            'D' => current.1 += steps,
            _   => todo!(),
        };
        area += (tmp.1 + current.1) * (tmp.0 - current.0);
        perimeter += steps;
    });

    area = area.abs() / 2;
    area += perimeter / 2;
    area += 1;

    area.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../../assets/test.txt").trim();
        assert_eq!(part1(input), "62");
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
