fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(part2(input));
}

const COLORS : [&str; 3] = ["red", "green", "blue"];

fn power_set(game: &str) -> i32 {
    let f = |[a, b, c] : [i32;3], [d, e, g] : [i32;3]| {
        [std::cmp::max(a, d), std::cmp::max(b, e), std::cmp::max(c, g)]
    };

    game.split(';').map(|c| {
        c.split(',').map(|c| {
            let mut r : [i32; 3] = [0, 0, 0];
            if let Some((count, color)) = c.trim().split_once(' ') {
                r[COLORS.iter().position(|&r| r == color).unwrap()] = count.parse::<i32>().unwrap();
            }
            r
            
        })
        .reduce(f).unwrap()
    }).reduce(f).unwrap()
    .iter().product()
}

fn part2(input: &str) -> String {
    let lines = input.lines();
    let result : i32 = lines.map(|l| {
        let mut game = l.split(':');
        let _id = game.next();
        power_set(game.next().unwrap())
    }).sum();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../../assets/test2.txt").trim();
        assert_eq!(part2(input), "2286");
    }
}
