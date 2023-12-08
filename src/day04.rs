pub struct Card {
    eql: usize,
}
impl Card {
    fn parse(line: &str) -> Card {
        let (win, own) = line
            .split(':')
            .last()
            .unwrap()
            .split_once('|')
            .unwrap();
        let win: Vec<usize> = win
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        let own: Vec<usize> = own
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        let eql = win.iter().filter(|num| {
            own.contains(num)
        }).count();

        Card { eql }
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Card> {
    input.lines().map(|line| Card::parse(line)).collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[Card]) -> usize {
    input
        .iter()
        .fold(0, |acc, card| {
            if card.eql >= 1{
                acc + 2_usize.pow(card.eql as u32 - 1)
            } else {
                acc
            }
        })
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[Card]) -> usize {
    let mut copies = vec![1_usize; input.len()];
    let len = input.len() - 1;

    for (idx, card) in input.iter().enumerate() {
        let win = if card.eql > len {
            len
        } else {
            card.eql + idx
        };

        for n in idx + 1..=win {
            copies[n] += copies[idx];
        }
    }

    copies.iter().sum()
}