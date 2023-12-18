use std::cmp::Ordering;
use std::collections::HashMap;

type Cards = HashMap<char, usize>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

enum Part {
    One,
    Two,
}

fn value(card: char, part: Part) -> usize {
    match card {
        card @ '2'..='9' => card.to_digit(10).unwrap()as usize,
        'T' => 10,
        'J' => match part {
            Part::One => 11,
            Part::Two => 1,
        },
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        card => panic!("Not a recognised card: {}", card),
    }
}

// Such a descriptive function name. Well done!
fn rainman(hand: &Vec<char>) -> Cards {
    let mut count: Cards = HashMap::new();

    hand.iter().for_each(|&card| {
        *count
            .entry(card)
            .or_insert(0) += 1;
    });

    count
}

fn detect_type(hand: Cards, part: Part) -> HandType {
    let mut same: Vec<usize> = hand.values().copied().collect();
    same.sort_unstable_by(|a, b| b.cmp(a));

    match part {
        Part::One => {},
        Part::Two => {
            if let Some(jokers) = hand.get(&'J') {
                if let Some(index) = same.iter().position(|&j| j == *jokers) {
                    if same.len() > 1 {
                        same.remove(index);
                        same[0] += jokers;
                    }
                }
            }
        }
    }

    match same.as_slice() {
        [5]       => HandType::FiveOfAKind,
        [4, 1]    => HandType::FourOfAKind,
        [3, 2]    => HandType::FullHouse,
        [3, ..]   => HandType::ThreeOfAKind,
        [2, 2, 1] => HandType::TwoPair,
        [2, ..]   => HandType::OnePair,
        [1, ..]   => HandType::HighCard,
        _ => panic!("Error in card count"),
    }
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<(Vec<char>, usize)> {
    input
        .lines()
        .map(|line| {
            line
                .trim()
                .split_once(' ')
                .map(|(cards, bet)| (
                    cards.chars().collect(),
                    bet.parse().unwrap()
                ))
                .unwrap()
        })
        .collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &Vec<(Vec<char>, usize)>) -> usize {
    let mut hands: Vec<(&Vec<char>, &usize, HandType)> = Vec::new();
    
    for (hand, bid) in input {
        let counted = rainman(hand);
        let handtype = detect_type(counted, Part::One);
        hands.push((hand, bid, handtype));
    }

    hands.sort_by(|(vec1, _, handtype1), (vec2, _, handtype2)| {
        match handtype1.cmp(handtype2) {
            Ordering::Equal => {
                vec1.iter().map(|&card| value(card, Part::One)).cmp(vec2.iter().map(|&card| value(card, Part::One)))
            }
            other => other,
        }
    });

    let score: usize = hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.1)
        .sum();

    score
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &Vec<(Vec<char>, usize)>) -> usize {
    let mut hands: Vec<(&Vec<char>, &usize, HandType)> = Vec::new();

    for (hand, bid) in input {
        let counted = rainman(hand);
        let handtype = detect_type(counted, Part::Two);
        hands.push((hand, bid, handtype));
    }

    hands.sort_by(|(vec1, _, handtype1), (vec2, _, handtype2)| {
        match handtype1.cmp(handtype2) {
            Ordering::Equal => {
                vec1.iter().map(|&card| value(card, Part::Two)).cmp(vec2.iter().map(|&card| value(card, Part::Two)))
            }
            other => other,
        }
    });

    let score: usize = hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.1)
        .sum();

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483";

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&input_generator(TEST)), 6440);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&input_generator(TEST)), 5905);
    }
}