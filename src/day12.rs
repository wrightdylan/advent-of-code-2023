use rayon::prelude::*;
use std::collections::HashMap;
use std::iter::repeat;

type Record = (Vec<Symbol>, Vec<usize>);

// The alphabet of the NFA
#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    Operational,
    Damaged,
    Unknown,
}

trait NFA {
    fn permutations(&self) -> usize;
    fn unfold(&self) -> Record;
}

impl NFA for Record {
    fn permutations(&self) -> usize {
        let symbols = &self.0;
        let groups = &self.1;
        let mut sequence = vec![Symbol::Operational];
        for &size in groups {
            for _ in 0..size {
                sequence.push(Symbol::Damaged);
            }
            sequence.push(Symbol::Operational);
        }
    
        let mut powerset: HashMap<usize, usize> = HashMap::new();
        let mut subset: HashMap<usize, usize> = HashMap::new();
        powerset.insert(0, 1);
    
        for symbol in symbols.iter() {
            for (&state, &count) in &powerset {
                match symbol {
                    Symbol::Operational => {
                        if state + 1 < sequence.len() && sequence[state + 1] == Symbol::Operational {
                            *subset.entry(state + 1).or_default() += count;
                        }
                        if sequence[state] == Symbol::Operational {
                            *subset.entry(state).or_default() += count;
                        }
                    },
                    Symbol::Damaged     => {
                        if state + 1 < sequence.len() && sequence[state + 1] == Symbol::Damaged {
                            *subset.entry(state + 1).or_default() += count;
                        }
                    },
                    Symbol::Unknown     => {
                        if state + 1 < sequence.len() {
                            *subset.entry(state + 1).or_default() += count;
                        }
                        if sequence[state] == Symbol::Operational {
                            *subset.entry(state).or_default() += count;
                        }
                    },
                }
            }
    
            powerset = subset;
            subset = HashMap::new();
        }
    
        *powerset.get(&(sequence.len() - 1)).unwrap_or(&0)
            + *powerset.get(&(sequence.len() - 2)).unwrap_or(&0)
    }

    fn unfold(&self) -> Record {
        (
            (0..4).fold(self.0.clone(), |mut acc, _| {
                acc.push(Symbol::Unknown);
                acc.extend_from_slice(&self.0);
                acc
            }),
            repeat(&self.1)
                .take(5)
                .flatten()
                .cloned()
                .collect::<Vec<usize>>(),
        )
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Record> {
    input
        .lines()
        .map(|line| {
            line.trim()
                .split_once(' ')
                .map(|(springs, count)| (
                    springs
                        .chars()
                        .map(|state| match state {
                            '.' => Symbol::Operational,
                            '#' => Symbol::Damaged,
                            '?' => Symbol::Unknown,
                            _   => panic!("Unknown state.")
                        })
                        .collect(),
                    count
                        .split(',')
                        .map(|num| num.parse::<usize>().unwrap())
                        .collect()
                ))
                .unwrap()
        })
        .collect()
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &Vec<Record>) -> usize {
    input
        .par_iter()
        .map(|line| line.permutations())
        .sum()
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &Vec<Record>) -> usize {
    input
        .par_iter()
        .map(|line| line.unfold().permutations())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "???.### 1,1,3
                        .??..??...?##. 1,1,3
                        ?#?#?#?#?#?#?#? 1,3,1,6
                        ????.#...#... 4,1,1
                        ????.######..#####. 1,6,5
                        ?###???????? 3,2,1";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 21);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(TEST)), 525_152);
    }
}