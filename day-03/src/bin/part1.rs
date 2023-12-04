fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(part1(input));
}


#[derive(Debug)]
struct Range
{
    begin : usize,
    end : usize,
}


fn part1(input: &str) -> String {
    let lines = input.lines();
    let mut sum : i32 = 0;
    let mut previous_number_ranges : Vec<(Range, i32)> = vec![];
    let mut previous_symbols : Vec<usize> = vec![];
    for line in lines {
        // parse over the line and extrat number and symbols
        let mut i : usize = 0;
        let mut current_number_ranges : Vec<(Range, i32)> = vec![];
        let mut current_symbol_locations : Vec<usize> = vec![];
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
                current_number_ranges.push((Range{begin:start,end}, n));
            }
            else if line.chars().nth(i).unwrap() == '.' {
                i += 1;
            }
            else {
                current_symbol_locations.push(i);
                i += 1;
            }
        }
        let symbol_in_range = |r : &Range, s : &usize| {
            r.begin <= s + 1 && r.end > s - 1
        };
        sum += previous_number_ranges.iter().filter(|(r, _)| current_symbol_locations.iter().any(|s| symbol_in_range(r, s))).map(|(_, v)| v).sum::<i32>();
        sum += current_number_ranges.iter().filter(|(r, _)| previous_symbols.iter().any(|s| symbol_in_range(r, s))).map(|(_, v)| v).sum::<i32>();
        current_number_ranges = current_number_ranges.into_iter().filter(|(r, _)| !previous_symbols.iter().any(|s| symbol_in_range(r, s))).collect::<Vec<_>>();
        sum += current_number_ranges.iter().filter(|(r, _)| current_symbol_locations.iter().any(|s| symbol_in_range(r, s))).map(|(_, v)| v).sum::<i32>();
        dbg!(&current_number_ranges);
        dbg!(&current_symbol_locations);

        previous_number_ranges = current_number_ranges.into_iter().filter(|(r, _)| !current_symbol_locations.iter().any(|s| symbol_in_range(r, s))).collect::<Vec<_>>();
        previous_symbols = current_symbol_locations;
        dbg!(sum);
        // now consolidate
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../../assets/test.txt").trim();
        assert_eq!(part1(input), "4361");
    }
}
