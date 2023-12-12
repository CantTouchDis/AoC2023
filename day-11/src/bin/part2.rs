#![cfg_attr(feature = "unstable", feature(test))]
 use std::collections::BTreeSet;

#[cfg(not(test))]
fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(part2(input, 1000000));
}


fn part2(input: &str, expansion : i32) -> String {
    let lines = input.lines();
    let mut empty_columns : BTreeSet<i32> = (0..140).collect::<BTreeSet<i32>>();
    let mut empty_rows : BTreeSet<i32> = BTreeSet::new();
    // (x, y) locations
    let mut locations : Vec<(i32, i32)>= vec![];
    lines.enumerate().for_each(|(y, l)| {
        let mut galaxies = l.chars().enumerate().filter(|(_x, c)| { *c == '#' }).peekable();
        if galaxies.peek().is_none() {
            empty_rows.insert(y as i32);
        }
        else {
            galaxies.for_each(|(x, _)| {
                empty_columns.remove(&(x as i32));
                locations.push((x as i32, y as i32));
            });
        }
    });
    let mut sum_distances : u64 = 0;
    for a in 0..locations.len() {
        for b in (a+1)..locations.len() {
            let from_x = std::cmp::min(locations[a].0, locations[b].0);
            let from_y = std::cmp::min(locations[a].1, locations[b].1);
            let to_x = std::cmp::max(locations[a].0, locations[b].0);
            let to_y = std::cmp::max(locations[a].1, locations[b].1);
            let expantion_x = (from_x..to_x).collect::<BTreeSet<i32>>().intersection(&empty_columns).count() as i32;
            let expantion_y = (from_y..to_y).collect::<BTreeSet<i32>>().intersection(&empty_rows).count() as i32;
            let manhatten_distance = {
                (locations[a].0 - locations[b].0).abs() + (locations[a].1 - locations[b].1).abs()
            };

            sum_distances += manhatten_distance as u64 + ((expantion_x + expantion_y) * (expansion - 1)) as u64;
        }
    }
    
    sum_distances.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../../assets/test.txt").trim();
        assert_eq!(part2(input, 10), "1030");
        assert_eq!(part2(input, 100), "8410");
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
            let _ = part2(input, 1000000);
        });
    }
}
