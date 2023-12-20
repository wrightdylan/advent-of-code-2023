use num::integer::lcm;
use std::collections::HashMap;

type Network = HashMap<String, (String, String)>;

pub enum Direction {
    Left,
    Right,
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> (Vec<Direction>, Network) {
    input
        .split_once("\n\n")
        .map(|(dirs, network)| (
            dirs
                .chars()
                .map(|c| {
                    match c {
                        'L' => Direction::Left,
                        'R' => Direction::Right,
                        c   => panic!("Unknown direction {}.", c),
                    }
                })
                .collect(),
            network
                .lines()
                .map(|line| {
                    line.split_once(" = ")
                        .map(|(key, nodes)| (
                            key.to_string(),
                            nodes
                                .trim_matches(|c| c == '(' || c == ')')
                                .split_once(", ")
                                .map(|(left, right)| (
                                    left.to_string(),
                                    right.to_string()
                                ))
                                .unwrap()
                        ))
                        .unwrap()
                })
                .collect()
        ))
        .unwrap()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &(Vec<Direction>, Network)) -> usize {
    let mut cnode = String::from("AAA");
    let mut direction = input.0.iter().cycle();
    let mut steps = 0;

    while cnode != "ZZZ" {
        let (left, right) = input.1[&cnode].clone();
        cnode = match direction.next().unwrap() {
            Direction::Left  => left,
            Direction::Right => right,
        };

        steps += 1;
    };
    
    steps
}

// This isn't going to work. I can't wait until the heat death of the Universe.
// The final answer is in the order of 18.6 x 10^12
// #[aoc(day8, part2)]
// pub fn solve_part2(input: &(Vec<Direction>, Network)) -> usize {
//     let mut steps = 0;
//     let mut cnodes: Vec<String> = input.1
//         .clone()
//         .into_keys()
//         .filter(|k| k.ends_with('A'))
//         .collect();
//     let mut direction = input.0.iter().cycle();
    
//     while !cnodes.iter().all(|node| node.ends_with('Z')) {
//         let mut new_cnodes = Vec::new();
//         let turn = direction.next().unwrap();

//         for cnode in cnodes.iter_mut() {
//             let (left, right) = input.1.get(cnode).unwrap().clone();
//             let new_cnode = match turn {
//                 Direction::Left  => left,
//                 Direction::Right => right,
//             };
//             new_cnodes.push(new_cnode);
//         }
//         cnodes = new_cnodes;
//         steps += 1;
//     }
    
//     steps
// }

// Apparently the best course of action is to use LCM. This was kind of subtly hinted at in the puzzle.
// Six nodes end in 'A'. These are listed below with their cycle lengths:
// AAA 17287
// BBA 19632
// GPA 13771
// GTA 20803
// VDA 23147
// VSA 17873
// Fortunately mine syncs at step 0, whereas it seems others are not so lucky.
// Another thing of note is that the number of directions given is a prime number: 293.
#[aoc(day8, part2)]
pub fn solve_part2(input: &(Vec<Direction>, Network)) -> usize {
    let cnodes: Vec<String> = input.1
        .clone()
        .into_keys()
        .filter(|k| k.ends_with('A'))
        .collect();

    // LCM magic
    cnodes.iter()
        .map(|cnode| {
            let mut cnode = cnode.to_owned();
            let mut direction = input.0.iter().cycle();
            let mut steps = 0;

            while !cnode.ends_with('Z') {
                let (left, right) = input.1[&cnode].clone();
                cnode = match direction.next().unwrap() {
                    Direction::Left  => left,
                    Direction::Right => right,
                };
                steps += 1;
            }

            steps
        })
        .fold(1, lcm)
}