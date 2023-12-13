fn parse_num(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .skip(1)
        .map(|num| num.parse().unwrap())
        .collect()
}

fn parse_line(input: &str) -> usize {
    input
        .split_whitespace()
        .skip(1)
        .fold("".to_string(), |acc, s| acc + s)
        .parse()
        .unwrap()
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let (times, dists) = input
        .split_once("\n")
        .map(|(first, second)| (parse_num(first), parse_num(second)))
        .unwrap();
    
    times
        .iter()
        .zip(&dists)
        .map(|(&time, &dist)| {
            (1..time)
                .filter(|&t| (time - t) * t > dist)
                .count()
        })
        .product()
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    let (time, dist) = input
        .split_once("\n")
        .map(|(first, second)| (parse_line(first), parse_line(second)))
        .unwrap();
    
    (1..time)
        .filter(|t| (time - t) * t > dist)
        .count()
}

// Apparently I can use a quadratic equation
#[aoc(day6, part2, quad)]
pub fn part2_quad(input: &str) -> usize {
    let (time, dist) = input
        .split_once("\n")
        .map(|(first, second)| (parse_line(first), parse_line(second)))
        .unwrap();
    
    ((time.pow(2) - 4 * dist) as f64).sqrt() as usize
}