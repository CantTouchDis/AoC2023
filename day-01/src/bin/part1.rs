fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(part1(input));
}

fn part1(input: &str) -> String {
    let lines = input.split('\n');
    dbg!(&lines);
    let result : i32 = lines.map(|l| {
        dbg!(l);
        let first = l.find(|c : char| c.is_digit(10)).expect("first found");
        let second = l.rfind(|c : char| c.is_digit(10)).expect("second found");
        let mut value = "".to_string();
        value.push(l.chars().nth(first).unwrap());
        value.push(l.chars().nth(second).unwrap());
        value.parse::<i32>().unwrap()
    }).sum();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../../assets/test.txt").trim();
        assert_eq!(part1(input), "142");
    }
}
