#![cfg_attr(feature = "unstable", feature(test))]

#[cfg(not(test))]
fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(part1(input));
}

fn char_to_value(c: char) -> i32 {
    const RANKS : [char; 5] = ['T', 'J', 'Q', 'K', 'A'];
    if c.is_digit(10) {
        c.to_digit(10).unwrap() as i32
    } else {
        RANKS.iter().position(|&v| v == c).unwrap() as i32 + 10
    }
}

fn hand_to_value(hand: &str) -> i32 {
    let card_counts = hand.chars().fold([0 as i32; 15], |mut counts, c| {
        counts[char_to_value(c) as usize] += 1;
        counts
    });
    // add the computation for five/four/full/three/twoP/oneP/high
    let hand_type;
    if card_counts.contains(&5) {
        hand_type = 6;
    }
    else if card_counts.contains(&4) {
        hand_type = 5;
    }
    else if card_counts.contains(&3) && card_counts.contains(&2) {
        hand_type = 4;
    }
    else if card_counts.contains(&3) {
        hand_type = 3;
    }
    else {
        hand_type = match card_counts.iter().filter(|&&v| { v == 2 }).count() {
            2 => 2,
            1 => 1,
            _ => 0,
        }
    }

    let hand_value = hand.chars()
        .map(|c| char_to_value(c))
        .fold(hand_type, |acc, v| {
            acc * 16 + v
        });
    hand_value
}

fn part1(input: &str) -> String {
    let mut hands : Vec<_> = input.lines()
        .map(|line| {
            let mut it = line.split_ascii_whitespace();
            // transform the line into the hand and the bid
            let (hand, bid) = (it.next().unwrap(), it.next().unwrap().parse::<i32>().unwrap());

            (hand_to_value(hand), bid)
        }).collect();
    hands.sort();
    hands.iter().enumerate().map(|(i, (_, b))| (i as i32 + 1) * b).sum::<i32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../../assets/test.txt").trim();
        assert_eq!(part1(input), "6440");
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
            let _ = part1(input);
        });
    }
}
