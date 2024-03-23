use num::integer::lcm;
use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    fmt::Debug,
};

type Modules = RefCell<HashMap<String, Box<dyn Module>>>;
type Routes  = HashMap<String, Vec<String>>;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    #[default]
    Low,
    High,
}

impl Level {
    fn not(&self) -> Self {
        match self {
            Level::High => Level::Low,
            Level::Low  => Level::High,
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Broadcaster {
    state: Level,
}

impl Module for Broadcaster {
    fn clock(&mut self, signal: Level, _: String) {
        self.state = signal;
    }

    fn state(&self) -> Level {
        self.state
    }
}

// Essentially a NAND gate
#[derive(Debug, Default, Clone)]
struct Conjunction {
    inputs: HashMap<String, Level>,
    state: Level,
}

impl Module for Conjunction {
    fn add_input(&mut self, name: String) {
        self.inputs.insert(name, Level::Low);
    }

    fn clock(&mut self, signal: Level, name: String) {
        self.inputs.insert(name, signal);
        self.state = match self.inputs.values().all(|&input| input == Level::High) {
            true  => Level::Low,
            false => Level::High,
        };
    }

    fn init_cycles(&self) -> HashMap<String, Option<usize>> {
        self.inputs.keys()
            .map(|key| (key.clone(), None))
            .collect()
    }

    fn state(&self) -> Level {
        self.state
    }
}

#[derive(Debug, Default, Clone)]
struct Flipflop {
    state: Level,
}

impl Module for Flipflop {
    fn clock(&mut self, signal: Level, _: String) {
        if signal == Level::Low {
            self.state = self.state.not();
        }
    }

    fn latched(&self, signal: Level) -> bool {
        if signal == Level::High {
            return true;
        }
        false
    }

    fn state(&self) -> Level {
        self.state
    }
}

pub trait Module: Debug {
    fn add_input(&mut self, _: String) {}
    fn clock(&mut self, signal: Level, name: String);
    fn init_cycles(&self) -> HashMap<String, Option<usize>> { HashMap::new() }
    fn latched(&self, _: Level) -> bool { false }
    fn state(&self) -> Level;
}

trait Cycles {
    fn all_highs(&self) -> bool;
    fn lcm(&self) -> usize;
}

impl Cycles for HashMap<String, Option<usize>> {
    fn all_highs(&self) -> bool {
        self.values().all(|length| length.is_some())
    }

    fn lcm(&self) -> usize {
        self.values()
            .filter_map(|v| *v)
            .fold(1, |acc, v| lcm(acc, v))
    }
}

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> (Modules, Routes) {
    // HashMap of modules needs to be heterogeneous
    // https://simonewebdesign.it/rust-hashmap-insert-values-multiple-types/
    // Option #1 honestly feels a bit like enum abuse...
    // Use option #2, but with Debug rather than Display
    let modules = RefCell::new(HashMap::new());
    let mut outputs = HashMap::new();

    for line in input.lines() {
        let (source, drain_str) = line.trim().split_once(" -> ").unwrap();

        let (mod_type, mod_name): (Box<dyn Module>, &str) = match source.split_at(1) {
            ("b", _) => (Box::new(Broadcaster::default()), "broadcaster"),
            ("&", source) => (Box::new(Conjunction::default()), source),
            ("%", source) => (Box::new(Flipflop::default()), source),
            (m, _) => panic!("Not a recognised module {}.", m),
        };

        modules.borrow_mut().insert(mod_name.to_string(), mod_type);
        outputs.insert(mod_name.to_string(), drain_str.split(", ").map(String::from).collect());

        outputs.iter().for_each(|(tx, rx)| {
            for id in rx {
                modules.borrow_mut().get_mut(id).map(|cm| cm.add_input(tx.clone()));
            }
        });
    }

    (modules, outputs)
}

#[aoc(day20, part1)]
pub fn solve_part1((modules, routes): &(Modules, Routes)) -> usize {
    let mut pulses = (0, 0);
    for _ in 0..1000 {
        let mut queue = VecDeque::from(vec![("broadcaster".to_string(), Level::Low)]);
        pulses.0 += 1;

        while let Some((source, signal)) = queue.pop_front() {
            for name in routes.get(&source).unwrap() {
                match signal {
                    Level::High => pulses.1 += 1,
                    Level::Low  => pulses.0 += 1,
                }
                if let Some(drain) = modules.borrow_mut().get_mut(name) {
                    if drain.latched(signal) { continue }
                    drain.clock(signal, source.clone());
                    queue.push_back((name.to_string(), drain.state()))
                } else {
                    continue;
                }
            }
        }
    }
    
    pulses.0 * pulses.1
}

#[aoc(day20, part2)]
pub fn solve_part2((modules, routes): &(Modules, Routes)) -> usize {
    let mut cycle = 0;
    let last_con = routes
        .iter()
        .find(|(_, drains)| drains.contains(&"rx".to_string()))
        .map(|(key, _)| key.clone())
        .unwrap();

    let mut cycle_lengths = modules.borrow().get(&last_con).as_ref().unwrap().init_cycles();

    loop {
        let mut queue = VecDeque::from(vec![("broadcaster".to_string(), Level::Low)]);
        cycle += 1;

        while let Some((source, signal)) = queue.pop_front() {
            for name in routes.get(&source).unwrap() {
                if let Some(drain) = modules.borrow_mut().get_mut(name) {
                    if drain.latched(signal) { continue }
                    drain.clock(signal, source.clone());
                    if name == &last_con && signal == Level::High && cycle_lengths.get(&source).unwrap().is_none() {
                        cycle_lengths.insert(source.clone(), Some(cycle));
                    }
                    if cycle_lengths.all_highs() {
                        return cycle_lengths.lcm();
                    }
                    queue.push_back((name.to_string(), drain.state()))
                } else {
                    continue;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    const TEST2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[test]
    fn part1_test1() {
        assert_eq!(solve_part1(&input_generator(TEST1)), 32_000_000);
    }

    #[test]
    fn part1_test2() {
        assert_eq!(solve_part1(&input_generator(TEST2)), 11_687_500);
    }
}