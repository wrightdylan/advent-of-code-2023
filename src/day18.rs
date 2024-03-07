type Point = (isize, isize);

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Instruction {
    dir: Direction,
    steps: isize,
    dec_dir: Direction,
    dec_steps: isize,
}

trait Lagoon {
    fn find_volume(&self, decoded: bool) -> usize;
}

#[allow(unused_assignments)]
impl Lagoon for Vec<Instruction> {
    fn find_volume(&self, decoded: bool) -> usize {
        let mut start = (0, 0);
        let mut vertices = Vec::from([start]);
        let mut boundary = 0;
        let mut dir = Direction::Up;
        let mut steps = 0;
        for inst in self {
            if decoded {
                dir = inst.dec_dir;
                steps = inst.dec_steps;
            } else {
                dir = inst.dir;
                steps = inst.steps;
            }
            let next = match dir {
                Direction::Up    => (start.0, start.1 + steps),
                Direction::Down  => (start.0, start.1 - steps),
                Direction::Left  => (start.0 - steps, start.1),
                Direction::Right => (start.0 + steps, start.1),
            };
            vertices.push(next);
            start = next;
            boundary += steps;
        }
        vertices.shoestring().picks(boundary as usize) + boundary as usize
    }
}

trait Area {
    fn shoestring(&self) -> usize;
}

impl Area for Vec<Point> {
    fn shoestring(&self) -> usize {
        (self
            .windows(2)
            .fold(0, |acc, matrix|
                acc + (matrix[0].0 * matrix[1].1) - (matrix[1].0 * matrix[0].1)
            ) / 2).abs() as usize
     }
}

trait Interior {
    fn picks(&self, boundary: usize) -> usize;
}

impl Interior for usize {
    fn picks(&self, boundary: usize) -> usize {
        self + 1 - boundary/2
    }
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let (dir_str, rest) = line.trim().split_once(' ').unwrap();
            let dir = match dir_str {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Not a valid direction."),
            };
            let (steps_str, dir_hex) = rest.split_once(' ').unwrap();
            let steps = steps_str.parse::<isize>().unwrap();
            let digits = dir_hex.chars()
                .filter(|c| c.is_digit(16))
                .collect::<String>();
            let dec_dir = digits.chars().last().unwrap();
            let dec_dir = match dec_dir {
                '0' => Direction::Right,
                '1' => Direction::Down,
                '2' => Direction::Left,
                '3' => Direction::Up,
                _ => panic!("Not a valid direction."),
            };
            let dec_steps = isize::from_str_radix(&digits[..digits.len() - 1], 16).unwrap();
            Instruction { dir, steps, dec_dir, dec_steps }
        })
        .collect()
}

#[aoc(day18, part1)]
pub fn solve_part1(input: &Vec<Instruction>) -> usize {
    input.find_volume(false)
}

#[aoc(day18, part2)]
pub fn solve_part2(input: &Vec<Instruction>) -> usize {
    input.find_volume(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "R 6 (#70c710)
                        D 5 (#0dc571)
                        L 2 (#5713f0)
                        D 2 (#d2c081)
                        R 2 (#59c680)
                        D 2 (#411b91)
                        L 5 (#8ceee2)
                        U 2 (#caa173)
                        L 1 (#1b58a2)
                        U 2 (#caa171)
                        R 2 (#7807d2)
                        U 3 (#a77fa3)
                        L 2 (#015232)
                        U 2 (#7a21e3)";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 62);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(TEST)), 952_408_144_115);
    }
}