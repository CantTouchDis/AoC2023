#![cfg_attr(feature = "unstable", feature(test))]

use std::collections::HashMap;

#[cfg(not(test))]
fn main() {
    let input = include_str!("../../assets/input.txt").trim();
    dbg!(part2(input, "rx"));
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

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn part2(input: &str, output : &str) -> String {
    let mut to_rx = "";
    let mut modules = {
        let mut mods = input.lines().map(|l| {
            let mut node_dest = l.split_terminator(" -> ");
            let node = node_dest.next().unwrap();
            let destinations = node_dest.next().unwrap().split_terminator(", ").map(|r| r.trim()).collect::<Vec<_>>();
            if destinations.contains(&output) {
                assert_eq!(l.chars().next().unwrap(), '&');
                to_rx = &node[1..];
            }
            if let Some(node_name) = node.strip_prefix('%') {
                //FlipFlop
                (node_name, (Module::FlipFlop(Pulse::Low), destinations))
            } else if let Some(node_name) = node.strip_prefix('&') {
                // Conjunction
                (node_name, (Module::Conjunction(HashMap::new()), destinations))
            } else {
                (node, (Module::Broadcast, destinations))
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
    //println!("{modules:?} {to_rx}");

    // our check
    {
        let mut cycle_starts = HashMap::new();
        let mut cycles = HashMap::new();

        let mut i : u64 = 0;
        let num_kc = {
            if let Module::Conjunction(v) = &modules.get(to_rx).unwrap().0 {
                v.len()
            } else { 0 }
        };

        while cycles.len() != num_kc
        {
            let mut queue_signals = vec![("button", "broadcaster", Pulse::Low)];
            let mut index = 0;
            while index != queue_signals.len() {
                let (from, to, signal) = queue_signals[index];
                // found an output with low before finding cycles:
                if to == output && signal == Pulse::Low {
                    return i.to_string();
                }
                if let Some(entry) = modules.get_mut(&to) {
                    if to == to_rx && signal == Pulse::High {
                        if let Some(v) = cycle_starts.insert(from, i + 1) {
                            cycles.insert(from, (i+1) - v);
                        }
                    }
                    match &mut entry.0 {
                        Module::Broadcast => {
                            for d in &entry.1 {
                                queue_signals.push((to, d, signal));
                            }
                        }
                        Module::FlipFlop(v) => {
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
            i += 1;
        }
        //println!("{cycles:?}");
        cycles.values().fold(1, |acc, v| lcm(acc, *v as usize)).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("../../assets/test2.txt").trim();
        assert_eq!(part2(input, "output"), "0");
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
            let _ = part2(input, "rx");
        });
    }
}
