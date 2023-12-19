#![cfg_attr(feature = "unstable", feature(test))]

use std::collections::HashMap;

#[cfg(not(test))]
fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(part1(input));
}


#[derive(Debug)]
enum Condition<'a> {
    LT(i32,&'a str, &'a str),
    GT(i32,&'a str, &'a str),
    ALWAYS(&'a str),
}

fn part1(input: &str) -> String {
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
    lines_it.map(|l| {
        let l = &l[1..(l.len()-1)];
        let parts = l.split_terminator(',').map(|v| {
            (&v[0..1], v[2..v.len()].parse::<i32>().unwrap())
        }).collect::<HashMap<&str, i32>>();
        let mut current = "in";
        while let Some(next_rules) = workflows.get(current) {
            for rule in next_rules {
                match rule {
                    Condition::LT(val, part, next) => {
                        if parts.get(part).unwrap() < val {
                            current = next;
                            break;
                        }
                    },
                    Condition::GT(val, part, next) => {
                        if parts.get(part).unwrap() > val {
                            current = next;
                            break;
                        }
                    },
                    Condition::ALWAYS(next) => {
                        current = next;
                        break;
                    }
                };
            }
        }
        if current == "R" {
            0
        }
        else {
            parts.values().sum::<i32>()
        }
    }).sum::<i32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../../assets/test.txt").trim();
        assert_eq!(part1(input), "19114");
    }
}

#[cfg(all(feature = "unstable", test))]
mod bench {
    use super::*;
    extern crate test;
    use self::test::Bencher;

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(|| {
            let input = test::black_box(include_str!("../../assets/input.txt").trim());
            let _ = part1(input);
        });
    }
}
