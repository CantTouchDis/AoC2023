#![cfg_attr(feature = "unstable", feature(test))]

#[cfg(not(test))]
fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(part2(input));
}

fn advent_hash(line: &str) -> i32 {
    line.chars().map(|c| c as i32).fold(0, |acc, v| {
        (acc + v) * 17 % 256
    })
}

fn part2(input: &str) -> String {
    let input = input.split_terminator(',');
    let mut boxes : Vec<Vec<(&str, i32)>> = vec![];
    boxes.resize(256, vec![]);
    input.for_each(|v| {
        if v.ends_with("-") {
            // remove the box
            let value = &v[0..(v.len()-1)];
            let box_nr = advent_hash(value);
            boxes[box_nr as usize].retain(|&e| e.0 != value);
        }
        else {
            let mut it = v.split_terminator('=');
            let value = it.next().unwrap();
            let lens_nr = it.next().unwrap().parse::<i32>().unwrap();
            let box_nr = advent_hash(value);
            if let Some(i) = boxes[box_nr as usize].iter().position(|&e| e.0 == value) {
                boxes[box_nr as usize][i].1 = lens_nr;
            }
            else
            {
                boxes[box_nr as usize].push((value, lens_nr));
            }
        }
    });
    //compute score
    let result = boxes.iter().enumerate().map(|(i, v)| {
        (i + 1) *// box_nr
            v.iter().enumerate().map(|(i, v)| {
                (i + 1) * v.1 as usize
            }).sum::<usize>()
    }).sum::<usize>();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../../assets/test.txt").trim();
        assert_eq!(part2(input), "145");
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
