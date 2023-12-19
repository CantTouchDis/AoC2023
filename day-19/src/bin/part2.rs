#![cfg_attr(feature = "unstable", feature(test))]

use std::collections::HashMap;

#[cfg(not(test))]
fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(part2(input));
}

#[derive(Debug)]
enum Condition<'a> {
    LT(i32,&'a str, &'a str),
    GT(i32,&'a str, &'a str),
    ALWAYS(&'a str),
}

fn accumulate_acceptance(current: &str, value_ranges: &HashMap<&str, (i32, i32)>, workflows: &HashMap<&str, Vec<Condition>>) -> u64 {
    if current == "R" {
        return 0;
    }
    else if current == "A" {
        //println!("Accepting Ranges: {value_ranges:?}");
        return value_ranges.values().map(|(a, b)| {
            assert!(a < b);
            (b - a + 1) as u64
        }).product::<u64>();
    }
    else if let Some(next_rules) = workflows.get(current) {
        let mut sum_values = 0;
        let mut new_values = value_ranges.clone();
        for rule in next_rules {
            match rule {
                Condition::LT(val, part, next) => {
                    let values = value_ranges.get(part).unwrap();
                    // if we dont intersect the range just skip
                    if values.0 < *val {
                        new_values.insert(part, (values.0, std::cmp::min(values.1, *val - 1)));
                        sum_values += accumulate_acceptance(next, &new_values, workflows);
                        assert!(*val < values.1);
                        new_values.insert(part, (*val, values.1));
                    }
                    if values.1 < *val {
                        break;
                    }
                },
                Condition::GT(val, part, next) => {
                    let values = value_ranges.get(part).unwrap();
                    // if we dont intersect the range just skip
                    if values.1 > *val {
                        new_values.insert(part, (std::cmp::max(values.0, *val + 1), values.1));
                        sum_values += accumulate_acceptance(next, &new_values, workflows);
                        assert!(values.0 < *val);
                        new_values.insert(part, (values.0, *val));
                    }
                    if values.0 > *val {
                        break;
                    }
                },
                Condition::ALWAYS(next) => {
                    sum_values += accumulate_acceptance(next, &new_values, workflows);
                    break;
                }
            };
        }
        return sum_values;
    }
    todo!();
    return 0;
}

fn part2(input: &str) -> String {
    let mut lines_it = input.lines();
    let workflows = lines_it.by_ref().take_while(|l| !l.is_empty()).map(|l| {
        let mut it = l.split_terminator('{');
        let name = it.next().unwrap();
        let rules = it.next().unwrap();
        let rules = (&rules[..(rules.len() - 1)]).split_terminator(',')
            .map(|r| {
                if let Some(pos) = r.chars().position(|c| c == ':') {
                    //let 
                    let (rule, next) = r.split_at(pos);
                    let (part, value) = rule.split_at(1);
                    let value = value[1..].parse::<i32>().unwrap();
                    if r.chars().nth(1).unwrap() == '<' {
                        Condition::LT(value, part, &next[1..])
                    } else {
                        Condition::GT(value, part, &next[1..])
                    }
                }
                else {
                    Condition::ALWAYS(r)
                }
            })
            .collect::<Vec<_>>();


        (name, rules)
    }).collect::<HashMap<&str, _>>();
    //println!("{workflows:?}");
    // go through the workflows and keep track of a equation
    let ranges : HashMap<&str, (i32, i32)> = HashMap::from([
      ("x", (1,4000)),
      ("m", (1,4000)),
      ("a", (1,4000)),
      ("s", (1,4000)),
    ]);
    let x = 4000 as u64;
    let x = x * x * x * x;
    accumulate_acceptance("in", &ranges, &workflows).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../../assets/test.txt").trim();
        assert_eq!(part2(input), "167409079868000");
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
