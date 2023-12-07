#![cfg_attr(feature = "unstable", feature(test))]

#[cfg(not(test))]
fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(part2(input));
}

fn char_to_value(c: char) -> i32 {
    const RANKS : [char; 5] = ['T', 'J', 'Q', 'K', 'A'];
    if c.is_digit(10) {
        c.to_digit(10).unwrap() as i32
    }
    else if c == 'J' {
        1
    } else {
        RANKS.iter().position(|&v| v == c).unwrap() as i32 + 10
    }
}

fn hand_to_value(hand: &str) -> i32 {
    let mut card_counts = hand.chars().fold([0 as i32; 15], |mut counts, c| {
        counts[char_to_value(c) as usize] += 1;
        counts
    });
    // add the number of jokers to the highest value
    *card_counts.iter_mut().skip(2).max().unwrap() += card_counts[1];
    card_counts[1] = 0;


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

fn part2(input: &str) -> String {
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
        assert_eq!(part2(input), "5905");
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
            let _ = part2(input);
        });
    }
}
