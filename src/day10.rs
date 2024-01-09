use std::collections::HashMap;

// I really wanted to stick with usize, but oh well!
type Point = (i32, i32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    BendNE,
    BendNW,
    BendSE,
    BendSW,
    PipeEW,
    PipeNS,
    Start,
}

impl Tile {
    pub fn contains_char(self, c: char) -> bool {
        let name = match self {
            Tile::BendNE => "NE",
            Tile::BendNW => "NW",
            Tile::BendSE => "SE",
            Tile::BendSW => "SW",
            Tile::PipeEW => "EW",
            Tile::PipeNS => "NS",
            Tile::Start  => "",
        };

        name.contains(c)
    }
}

#[derive(Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
    Stop,
}

impl Direction {
    pub fn from(self, loc: &Point) -> Point {
        match self {
            Direction::North => (loc.0, loc.1 - 1),
            Direction::East  => (loc.0 + 1, loc.1),
            Direction::South => (loc.0, loc.1 + 1),
            Direction::West  => (loc.0 - 1, loc.1),
            Direction::Stop  => (loc.0, loc.1),
        }
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
            ) / 2) as usize
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

fn find_next(map: &HashMap<Point, Tile>, current: Point, last: Option<Direction>) -> Point {
    let mut next = None;
    
    if last.is_none() {
        let check = &[
            (current.0, current.1 - 1, 'S'),
            (current.0 + 1, current.1, 'W'),
            (current.0, current.1 + 1, 'N'),
            (current.0 - 1, current.1, 'E'),
        ];

        for &(col, row, dir) in check {
            if let Some(tile) = map.get(&(col, row)) {
                if tile.contains_char(dir) {
                    next = Some((col, row));
                    break;
                }
            }
        }
    } else {
        next = match map.get(&current).unwrap() {
            Tile::BendNE => {
                match last.unwrap() {
                    Direction::South => Some(Direction::East.from(&current)),
                    Direction::West  => Some(Direction::North.from(&current)),
                    _ => panic!("Invalid move."),
                }
            },
            Tile::BendNW => {
                match last.unwrap() {
                    Direction::South => Some(Direction::West.from(&current)),
                    Direction::East  => Some(Direction::North.from(&current)),
                    _ => panic!("Invalid move."),
                }
            },
            Tile::BendSE => {
                match last.unwrap() {
                    Direction::North => Some(Direction::East.from(&current)),
                    Direction::West  => Some(Direction::South.from(&current)),
                    _ => panic!("Invalid move."),
                }
            },
            Tile::BendSW => {
                match last.unwrap() {
                    Direction::North => Some(Direction::West.from(&current)),
                    Direction::East  => Some(Direction::South.from(&current)),
                    _ => panic!("Invalid move."),
                }
            },
            Tile::PipeEW => {
                match last.unwrap() {
                    Direction::East => Some(Direction::East.from(&current)),
                    Direction::West => Some(Direction::West.from(&current)),
                    _ => panic!("Invalid move."),
                }
            },
            Tile::PipeNS => {
                match last.unwrap() {
                    Direction::North => Some(Direction::North.from(&current)),
                    Direction::South => Some(Direction::South.from(&current)),
                    _ => panic!("Invalid move."),
                }
            },
            Tile::Start => Some(Direction::Stop.from(&current)),
        };
    }

    next.unwrap()
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> (HashMap<Point, Tile>, Point) {
    input
        .lines()
        .enumerate()
        .fold((HashMap::new(), (0, 0)), |(mut map, mut start), (row, line)| {
            line.trim()
                .chars()
                .enumerate()
                .for_each(|(col, char)| {
                    let tile = match char {
                        'L' => Tile::BendNE,
                        'J' => Tile::BendNW,
                        'F' => Tile::BendSE,
                        '7' => Tile::BendSW,
                        '-' => Tile::PipeEW,
                        '|' => Tile::PipeNS,
                        'S' => {
                            start = (col as i32, row as i32);
                            Tile::Start
                        },
                        _   => return,
                    };
                    map.insert((col as i32, row as i32), tile);
                });
                (map, start)
        })
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &(HashMap<Point, Tile>, Point)) -> usize {
    let mut last_move = None;
    let start = input.1;
    let mut current = start;
    let mut visited: Vec<Point> = Vec::new();

    loop {
        let next = find_next(&input.0, current, last_move);
        visited.push(next);
        let delta = (next.0 - current.0, next.1 - current.1);
        last_move = match delta {
            (0, -1) => Some(Direction::North),
            (1, 0)  => Some(Direction::East),
            (0, 1)  => Some(Direction::South),
            (-1, 0) => Some(Direction::West),
            (0, 0)  => break,
            _ => panic!("Invalid move {:?}.", delta)
        };
        current = next;
    }
    
    visited.len() / 2
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &(HashMap<Point, Tile>, Point)) -> usize {
    let mut last_move = None;
    let start = input.1;
    let mut current = start;
    // let mut vertices: Vec<Point> = vec![start];
    // let mut visited: Vec<Point> = Vec::new();
    let mut visited: Vec<Point> = vec![start];

    loop {
        let next = find_next(&input.0, current, last_move);

        // Apparently, calculating the area using all +13k vertices is still faster than using an optimised
        // vertex table of only the start point and bends.
        // if let Some(&tile) = input.0.get(&next) {
        //     if !(tile == Tile::PipeEW || tile == Tile::PipeNS) {
        //         vertices.push(next);
        //     }
        // }

        visited.push(next);
        let delta = (next.0 - current.0, next.1 - current.1);
        last_move = match delta {
            (0, -1) => Some(Direction::North),
            (1, 0)  => Some(Direction::East),
            (0, 1)  => Some(Direction::South),
            (-1, 0) => Some(Direction::West),
            (0, 0)  => break,
            _ => panic!("Invalid move {:?}.", delta)
        };
        current = next;
    }
    // vertices.shoestring().picks(visited.len())
    visited.shoestring().picks(visited.len() - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &str = ".....
                        .S-7.
                        .|.|.
                        .L-J.
                        .....";

    const TEST2: &str = "..F7.
                        .FJ|.
                        SJ.L7
                        |F--J
                        LJ...";
    
    #[test]
    fn part1_test1() {
        assert_eq!(solve_part1(&input_generator(TEST1)), 4);
    }

    #[test]
    fn part1_test2() {
        assert_eq!(solve_part1(&input_generator(TEST2)), 8);
    }

    #[test]
    fn part2_test1() {
        assert_eq!(solve_part2(&input_generator(TEST1)), 1);
    }

    #[test]
    fn part2_test2() {
        assert_eq!(solve_part2(&input_generator(TEST2)), 1);
    }
}