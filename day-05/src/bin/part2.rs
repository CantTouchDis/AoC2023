#![cfg_attr(feature = "unstable", feature(test))]

#[cfg(not(test))]
fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(part2(input));
}

fn part2(input: &str) -> String {
    let mut lines = input.lines();
    let seeds : Vec<u64> = lines.next().unwrap().split_at(6).1.split_ascii_whitespace().map(|s| s.parse::<u64>().unwrap()).collect();
    let mut i = 0;
    let mut ranges = vec![];
    while i < seeds.len() {
        ranges.push((seeds[i], seeds[i]+seeds[i+1]));
        i += 2;
    }
    // skip empty line
    lines.next();
    let mut previous : Vec<(u64, u64)> = ranges.clone();
    let mut current : Vec<(u64, u64)> = vec![];
    for l in lines {
        if l.is_empty() {

        }
        else if !l.chars().nth(0).unwrap().is_digit(10) {
            // start a new map
            previous.append(&mut current);
            current = vec![];
            //println!("At start of {} we got {:?}", l, previous);
        }
        else {
            let m = l.split_ascii_whitespace().map(|a| a.parse::<u64>().unwrap() ).collect::<Vec<u64>>();
            let mut new_previous: Vec<(u64, u64)> = vec![];

            // aliases to keep my sanity
            let (dst, src_start, src_end, map_length) = (m[0], m[1], m[1] + m[2], m[2]);

            for (b, e) in &previous {
                // end before begin or begin after end
                if *e <= src_start || *b >= src_end {
                    new_previous.push((*b, *e));
                    continue;
                }
                // full overlap
                if *b >= src_start && *e <= src_end {
                    current.push((dst + *b - src_start, dst + *e - src_start));
                }
                // fully contained
                else if *b < src_start && *e > src_end {
                    new_previous.push((*b, src_start));
                    new_previous.push((src_end, *e));
                    current.push((dst, dst + map_length));
                }
                // begin before begin
                else if *b < src_start {
                    new_previous.push((*b, src_start));
                    current.push((dst, dst + *e - src_start));
                }
                else if *e > src_end {
                    new_previous.push((src_end, *e));
                    current.push((dst + *b - src_start, dst + map_length));
                }
                else {
                    println!("{} {} {} unhandled for range ({}, {})", m[0], m[1], m[2], *b, *e);
                }
            }
            previous = new_previous;
        }
    }
    previous.append(&mut current);
    //println!("We got {:?} in the end", previous);
    previous.iter().map(|(a, _)| a).reduce(|a, b| std::cmp::min(a, b)).unwrap().to_string()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../../assets/test.txt").trim();
        assert_eq!(part2(input), "46");
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
            let _ = part2(input);
        });
    }
}
