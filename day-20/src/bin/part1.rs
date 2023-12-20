#![cfg_attr(feature = "unstable", feature(test))]

use std::collections::HashMap;

#[cfg(not(test))]
fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(part1(input));
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug)]
enum Module<'a> {
    FlipFlop(Pulse),
    Conjunction(HashMap<&'a str, Pulse>),
    Broadcast,
}

fn part1(input: &str) -> String {
    let modules = {
        let mut mods = input.lines().map(|l| {
            let mut node_dest = l.split_terminator("->");
            let node = node_dest.next().unwrap();
            let destinations = node_dest.next().unwrap().split_terminator(", ").map(|r| r.trim()).collect::<Vec<_>>();
            match l.chars().nth(0).unwrap() {
                '%' => {
                    //FlipFlop
                    (&node[1..node.len()-1], (Module::FlipFlop(Pulse::Low), destinations))
                },
                '&' => {
                    // Conjunction
                    (&node[1..node.len()-1], (Module::Conjunction(HashMap::new()), destinations))
                },
                'b' => {
                    // broadcaster
                    (&node[..node.len()-1], (Module::Broadcast, destinations))
                },
                _   => panic!()
            }
        }).collect::<HashMap<&str, (Module, Vec<_>)>>();

        let mut conj_module_initializer : HashMap<&str, Vec<&str>> = HashMap::new();

        for (input, (_t, destinations)) in &mods {
            for dest in destinations {
                if let Some((Module::Conjunction(_), _)) = mods.get(dest) {
                   if conj_module_initializer.contains_key(dest) {
                        conj_module_initializer.get_mut(dest).unwrap().push(input);
                   }
                   else {
                        conj_module_initializer.insert(dest, vec![input]);
                   }
                }

            }
        }
        for (name, inputs) in conj_module_initializer {
            mods.get_mut(name).unwrap().0 = Module::Conjunction(
                inputs.iter().map(|i| (*i, Pulse::Low)).collect::<HashMap<_, _>>()
            );
        }

        mods
    };

    // our check
    {
        let mut local_modules : HashMap<&str, (Module, Vec<&str>)> = HashMap::new();
        local_modules.extend((modules).into_iter());

        let mut sum_low = 0;
        let mut sum_high = 0;

        for _i in 0..1000
        {
            let mut queue_signals = vec![("button", "broadcaster", Pulse::Low)];
            let mut index = 0;
            while index != queue_signals.len() {
                let (from, to, signal) = queue_signals[index];
                if let Pulse::Low = signal {
                    sum_low += 1;
                }
                else {
                    sum_high += 1;
                }
                //println!("{} -{:?}-> {:?}", from, signal, to);
                if let Some(entry) = local_modules.get_mut(&to) {
                    match &mut entry.0 {
                        Module::Broadcast => {
                            for d in &entry.1 {
                                queue_signals.push((to, d, signal));
                            }
                        }
                        Module::FlipFlop(ref mut v) => {
                            if let Pulse::Low = signal {
                                // flip the flipflop and send the update
                                let out_signal = if let Pulse::Low = v {
                                    Pulse::High
                                }
                                else
                                {
                                    Pulse::Low
                                };
                                for d in &entry.1 {
                                    queue_signals.push((to, d, out_signal));
                                }
                                *v = out_signal;
                            }
                        },
                        Module::Conjunction(ref mut m) => {
                            m.insert(from, signal);
                            let out_signal = if m.iter().all(|(_k, v)| *v == Pulse::High) {
                                Pulse::Low
                            }
                            else {
                                Pulse::High
                            };
                            for d in &entry.1 {
                                queue_signals.push((to, d, out_signal));
                            }
                        }
                    }
                }
                index += 1;
            }
        }
        println!("Low: {}, High: {}", sum_low, sum_high);
        return (sum_high * sum_low).to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../../assets/test.txt").trim();
        assert_eq!(part1(input), "32000000");
        println!("");
        let input = include_str!("../../assets/test2.txt").trim();
        assert_eq!(part1(input), "11687500");
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
