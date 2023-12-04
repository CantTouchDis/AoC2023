fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(part1(input));
}

fn is_possible(game: &str) -> bool {
    game.split(';').all(|g| {
        g.split(',').map(|c| c.trim()).all(|c| {
            if let Some((count, color)) = c.split_once(' ') {
                CONDITION.iter().find(|e| e.0 == color).unwrap().1 >= count.parse::<i32>().unwrap()
            }
            else
            {
                true
            }
        })
    })
}

const CONDITION: [(&str, i32); 3] = [
    ("red", 12),
    ("green", 13),
    ("blue", 14),
];

fn part1(input: &str) -> String {
    let lines = input.lines();
    dbg!(&lines);
    let result : i32 = lines.map(|l| {
        let mut game = l.split(':');
        let id = game.next()
             .unwrap()
             .split(' ')
             .nth(1)
             .unwrap()
             .parse::<i32>()
             .unwrap();
        if is_possible(game.next().unwrap()) {
            id
        }
        else {
            0
        }
        
    }).sum();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../../assets/test.txt").trim();
        assert_eq!(part1(input), "8");
    }
}
