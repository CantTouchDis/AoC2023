#![cfg_attr(feature = "unstable", feature(test))]

#[cfg(not(test))]
fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(part2(input));
}

fn part2(input: &str) -> String {
    let mut current : (i128, i128) = (0, 0);
    let mut area : i128 = 0;
    let mut perimeter : i128 = 0;
    input.lines()
        .map(|l| {
            let mut plan_line = l.split_ascii_whitespace();
            // skip the first two
            plan_line.next();
            plan_line.next();

            let hex_value = plan_line.next().unwrap();
            let steps = i128::from_str_radix(&hex_value[2..7], 16).unwrap();
            let dir = match hex_value.chars().nth(7).unwrap() {
                '0' => 'R',
                '1' => 'D',
                '2' => 'L',
                '3' => 'U',
                _   => todo!(),
            };
            println!("{} {}", dir, steps);


            (dir, steps)
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
        assert_eq!(part2(input), "952408144115");
    }
}

#[cfg(all(feature = "unstable", test))]
mod bench {
    use super::*;
    extern crate test;
    use self::test::Bencher;

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = test::black_box(include_str!("../../assets/input.txt").trim());
        b.iter(move || {
            //std::thread::sleep(std::time::Duration::from_nanos(1000));
            let _ = part2(input);
        });
    }
}
