pub struct Parser {
    pub simple: Vec<String>,
    pub steps: Vec<(String, Operation)>
}

impl Parser {
    fn hashmapper(&self) -> Vec<Vec<Lens>> {
        let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256]; 

        for (label, operation) in self.steps.iter() {
            let box_num = hasher(label);

            match operation {
                Operation::Insert(folen) => {
                    if let Some(lens) = boxes[box_num].iter_mut().find(|bx| bx.label == *label) {
                        lens.folen = *folen;
                    } else {
                        boxes[box_num].push( Lens { label: label.clone(), folen: *folen } );
                    }
                },
                Operation::Remove => {
                    if let Some(index) = boxes[box_num].iter().position(|bx| bx.label == *label) {
                        boxes[box_num].remove(index);
                    }
                },
            }
        }
        
        boxes
    }

    fn hash_score(&self) -> usize {
        self.simple
            .iter()
            .map(|step| hasher(step))
            .sum()
    }
}

pub enum Operation {
    Insert(usize),
    Remove
}

#[derive(Debug, Clone)]
pub struct Lens {
    label: String,
    folen: usize,
}

trait LensBox {
    fn focusing_power(&self) -> usize;
}

impl LensBox for Vec<Vec<Lens>> {
    fn focusing_power(&self) -> usize {
        self.iter()
            .enumerate()
            .fold(0, |power, (bx, slots)| {
                let box_score = bx + 1;
                power + slots
                    .iter()
                    .enumerate()
                    .map(|(slot, lens)| (slot + 1) * lens.folen)
                    .sum::<usize>()
                    * box_score
            })
    }
}

pub fn hasher(label: &str) -> usize {
    label.chars()
        .fold(0, |cv, char| (cv + char as usize) * 17 % 256)
}

pub fn initialiser(step: &str) -> (String, Operation) {
    if step.ends_with('-') {
        let label = step[..step.len() - 1].to_string();
        (label, Operation::Remove)
    } else if let Some((label, num)) = step.split_once('=') {
        (label.to_string(), Operation::Insert(num.parse::<usize>().unwrap()))
    } else {
        panic!("Invalid instruction.");
    }
}

#[aoc_generator(day15)]
// Modified for Part 2 to offload processing to the generator. There's a bit too much redundancy for my liking.
pub fn input_generator(input: &str) -> Parser {
    Parser {
        simple: input
            .lines()
            .flat_map(|line| line.trim().split(',').map(String::from))
            .collect(),
        steps: input
            .lines()
            .flat_map(|line| line.trim().split(',').map(String::from))
            .map(|step| initialiser(&step))
            .collect(),
    }
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &Parser) -> usize {
    input.hash_score()
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &Parser) -> usize {
    input.hashmapper().focusing_power()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn hash_test() {
        assert_eq!(hasher("rn=1"), 30);
        assert_eq!(hasher("cm-"), 253);
        assert_eq!(hasher("qp=3"), 97);
        assert_eq!(hasher("cm=2"), 47);
    }

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 1320);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(TEST)), 145);
    }
}