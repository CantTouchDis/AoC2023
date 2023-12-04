fn main() {
    let input = include_str!("../../assets/input.txt");
    dbg!(part2(input));
}

fn convert_input(input: &str) -> i32 {
    let mut result : Vec<i32> = vec![];
    for (i, c) in input.chars().enumerate() {
        if c.is_digit(10) {
            result.push(String::from(c).parse::<i32>().unwrap());
        }
        else
        {
            match &input[i..] {
                s if s.starts_with("zero")  => result.push(0),
                s if s.starts_with("one")   => result.push(1),
                s if s.starts_with("two")   => result.push(2),
                s if s.starts_with("three") => result.push(3),
                s if s.starts_with("four")  => result.push(4),
                s if s.starts_with("five")  => result.push(5),
                s if s.starts_with("six")   => result.push(6),
                s if s.starts_with("seven") => result.push(7),
                s if s.starts_with("eight") => result.push(8),
                s if s.starts_with("nine")  => result.push(9),
                _ => (),
            }
        }
    }
    result[0] * 10 + result[result.len()-1]
}

fn part2(input: &str) -> String {
    let lines = input.lines();
    let result : i32 = lines.map(convert_input).sum();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../../assets/test2.txt");
        assert_eq!(part2(input), "281");
    }
}
