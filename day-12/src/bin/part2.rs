#![cfg_attr(feature = "unstable", feature(test))]

use std::collections::HashMap;

#[cfg(not(test))]
fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(part2(input));
}


fn fit_group(gears: &str, group_size : usize) -> bool {
    gears.len() >= group_size &&
        (&gears[..group_size]).chars().filter(|c| *c == '.').count() == 0 &&
            gears.chars().nth(group_size).map_or(true, |c| c != '#')
}

fn num_working<'a>(gears: &'a str, groups: &'a [i32], cache : &mut HashMap<(&'a str, &'a [i32]), u128>, free_spaces: Option<i32>) -> u128 {
    // check if the previous alignment works
    // empty group can only fit into "empty" space
    // all '?' become ' '
    if groups.len() == 0 {
        return if gears.chars().any(|c| c == '#') {
            0
        } else {
            1
        }
    }
    // we cant work with negative spaces. the previous alignment is invalid
    let free_spaces = if let Some(free) = free_spaces
    {
        free
    }
    else
    {
        let blocked_spaces = groups.iter().map(|v| v + 1).sum::<i32>() - 1;
        gears.len() as i32 - blocked_spaces
    };
    if free_spaces < 0
    {
        return 0;
    }

    let mut count : u128 = 0;
    for offset in 0..=(free_spaces as usize) {
        if fit_group(&gears[offset..], groups[0] as usize) {
            if offset + groups[0] as usize + 1 >= gears.len() {
                count += 1;
            } else {
                if let Some(v) = cache.get(&(&gears[(offset + groups[0] as usize + 1)..], &groups[1..])) {
                    count += v;
                } else
                {
                    let result = num_working(&gears[(offset + groups[0] as usize + 1)..], &groups[1..], cache, Some(free_spaces - offset as i32));
                    cache.insert((&gears[(offset + groups[0] as usize + 1)..], &groups[1..]), result);
                    count += result;
                }
            }
        }
        // if the current first character is a '#' we have no other option as to start a group
        if gears.chars().nth(offset).map_or(false, |c| c == '#') {
            break;
        }
    }
    count
}


fn part2(input: &str) -> String {
    let lines = input.lines();
    let gear_groups = lines.map(|l| {
        let mut it = l.split_ascii_whitespace();
        let mut gears = it.next().unwrap().to_string();
        gears.push('?');
        let mut gears = gears.repeat(5);
        gears.pop();
        let groups = it.next().unwrap().split(',').map(|c| c.parse::<i32>().unwrap()).collect::<Vec<_>>();
        let groups = groups.repeat(5);
        (gears, groups)
    }).collect::<Vec<_>>();

    let result = gear_groups.iter().map(|(gears, groups)| {
        num_working(gears, groups, &mut HashMap::new(), None)
    }).sum::<u128>();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../../assets/test.txt").trim();
        assert_eq!(part2(input), "525152");
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
