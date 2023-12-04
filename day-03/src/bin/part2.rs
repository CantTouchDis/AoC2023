fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(part2(input));
}

#[derive(Debug)]
struct Range
{
    begin : usize,
    end : usize,
}

fn part2(input: &str) -> String {
    let lines = input.lines();
    let mut number_ranges : Vec<Vec<(Range, i32)>> = vec![];
    let mut ast_locs : Vec<Vec<usize>> = vec![];
    for line in lines {
        let mut i = 0;
        let mut current_numbers = vec![];
        let mut current_locs = vec![];
        while i < line.len() {
            if line.chars().nth(i).unwrap().is_digit(10) {
                let mut number = "".to_string();
                let start = i;
                while i < line.len() && line.chars().nth(i).unwrap().is_digit(10) {
                    number.push(line.chars().nth(i).unwrap());
                    i += 1;
                }
                let end = i;
                let n = number.parse::<i32>().unwrap();
                current_numbers.push((Range{begin:start,end}, n));
            }
            else if line.chars().nth(i).unwrap() == '.' {
                i += 1;
            }
            else {
                current_locs.push(i);
                i += 1;
            }
        }
        number_ranges.push(current_numbers);
        ast_locs.push(current_locs);
    }
    let mut sum = 0;
    let mut i = 0;
    let symbol_in_range = |r : &Range, s : &usize| {
        r.begin <= s + 1 && r.end > s - 1
    };
    while i < ast_locs.len() {
        sum += ast_locs[i].iter().map(|s| {
            // find numbers in ranges i-1 i+1 and i
            let mut it = number_ranges[i - 1].iter().filter(|(r, _)| symbol_in_range(r, s))
                .chain(
                    number_ranges[i].iter().filter(|(r, _)| symbol_in_range(r, s)))
                .chain(
                    number_ranges[i + 1].iter().filter(|(r, _)| symbol_in_range(r, s)));
            let first = it.next();
            let second = it.next();
            let third = it.next();
            match (first, second, third) {
                (Some(f), Some(s), None) => f.1 * s.1,
                _                        => 0,
            }
        }).sum::<i32>();
        i += 1;
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../../assets/test2.txt").trim();
        assert_eq!(part2(input), "467835");
    }
}
