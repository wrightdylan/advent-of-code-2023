use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::{HashMap, HashSet};

type Point = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

enum Turn {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Feature {
    Empty,       //  '.'
    OccMirror,   //  '\'
    OriMirror,   //  '/'
    HSplitter,   //  '-'
    VSplitter,   //  '|'
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tile {
    feature: Feature,
    visited: Vec<Direction>,
}

impl Tile {
    fn new(feature: Feature) -> Self {
        Tile { feature, visited: Vec::new() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heading {
    pos: Point,
    dir: Direction,
}

impl Heading {
    fn advance(&self, max: &Point) -> Option<Heading> {
        let pos = match self.dir {
            Direction::North if self.pos.1 > 0 => Some((self.pos.0, self.pos.1 - 1)),
            Direction::East if self.pos.0 < max.0 => Some((self.pos.0 + 1, self.pos.1)),
            Direction::South if self.pos.1 < max.1 => Some((self.pos.0, self.pos.1 + 1)),
            Direction::West if self.pos.0 > 0 => Some((self.pos.0 - 1, self.pos.1)),
            _ => None,
        };

        if pos.is_some() {
            return Some(Heading::from(pos.unwrap(), self.dir));
        }

        None
    }

    fn from(pos: (usize, usize), dir: Direction) -> Self {
        Heading { pos, dir }
    }

    fn turn(&self, turn: Turn) -> Self {
        let new_dir = match (self.dir, turn) {
            (Direction::North, Turn::Left)  => Direction::West,
            (Direction::North, Turn::Right) => Direction::East,
            (Direction::East, Turn::Left)   => Direction::North,
            (Direction::East, Turn::Right)  => Direction::South,
            (Direction::South, Turn::Left)  => Direction::East,
            (Direction::South, Turn::Right) => Direction::West,
            (Direction::West, Turn::Left)   => Direction::South,
            (Direction::West, Turn::Right)  => Direction::North,
        };

        Heading { pos: self.pos, dir: new_dir }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Floor {
    map: HashMap<Point, Tile>,
    max: Point,  // Reminder: this is zero-indexed
}

impl Floor {
    fn append_dir(&mut self, marker: &Heading) {
        let mut tile = self.map.get(&marker.pos).unwrap().clone();
        tile.visited.push(marker.dir);
        self.map.insert(marker.pos, tile);
    }

    fn explore(mut self, start: Heading) -> HashSet<Point> {
        let mut branch_queue = Vec::from([start.clone()]);
        let mut visited = HashSet::new();

        while let Some(mut marker) = branch_queue.pop() {
            while !self.map.get(&marker.pos).unwrap().visited.contains(&marker.dir) {
                visited.insert(marker.pos);
                self.append_dir(&marker);

                match self.map.get(&marker.pos).unwrap().feature {
                    Feature::Empty     => if let Some(next_pos) = marker.advance(&self.max) {
                        marker = next_pos;
                    } else {
                        break
                    },
                    Feature::OccMirror => {
                        marker = match marker.dir {
                            Direction::North | Direction::South => marker.turn(Turn::Left),
                            Direction::East  | Direction::West  => marker.turn(Turn::Right),
                        };
                        if let Some(next_pos) = marker.advance(&self.max) {
                            marker = next_pos;
                        } else {
                            break
                        }
                    },
                    Feature::OriMirror => {
                        marker = match marker.dir {
                            Direction::North | Direction::South => marker.turn(Turn::Right),
                            Direction::East  | Direction::West  => marker.turn(Turn::Left),
                        };
                        if let Some(next_pos) = marker.advance(&self.max) {
                            marker = next_pos;
                        } else {
                            break
                        }
                    },
                    Feature::HSplitter => {
                        if let Direction::North | Direction::South = marker.dir {
                            if let Some(branch1) = marker.turn(Turn::Left).advance(&self.max) {
                                branch_queue.push(branch1);
                            }
                            if let Some(branch2) = marker.turn(Turn::Right).advance(&self.max) {
                                branch_queue.push(branch2);
                            }
                            break;
                        } else if let Some(next_pos) = marker.advance(&self.max) {
                            marker = next_pos;
                        } else {
                            break
                        }
                    },
                    Feature::VSplitter => {
                        if let Direction::East | Direction::West = marker.dir {
                            if let Some(branch1) = marker.turn(Turn::Left).advance(&self.max) {
                                branch_queue.push(branch1);
                            }
                            if let Some(branch2) = marker.turn(Turn::Right).advance(&self.max) {
                                branch_queue.push(branch2);
                            }
                            break;
                        } else if let Some(next_pos) = marker.advance(&self.max) {
                            marker = next_pos;
                        } else {
                            break
                        }
                    },
                }
            }
        }

        visited
    }
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Floor {
    let map: HashMap<Point, Tile> = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(move |(col, ch)| {
                    ((col, row), Tile::new(match ch {
                        '.'  => Feature::Empty,
                        '\\' => Feature::OccMirror,
                        '/'  => Feature::OriMirror,
                        '-'  => Feature::HSplitter,
                        '|'  => Feature::VSplitter,
                        _    => panic!("Unrecognised tile type: {}", ch),
                    }))
            })
    }).collect();

    let max = map.keys().cloned().max().unwrap_or((0, 0));

    Floor { map, max }
}

// LOL! This worked perfectly on the first attempt
#[aoc(day16, part1)]
pub fn solve_part1(input: &Floor) -> usize {
    let start = Heading::from((0, 0), Direction::East);
    input.clone().explore(start).len()
}

// Lucky for part 2, I initially wrote part 1 with the ability to start from any position
#[aoc(day16, part2)]
pub fn solve_part2(input: &Floor) -> usize {
    let candidates: Vec<Heading> = (0..=input.max.0)
        .flat_map(|i| {
            vec![
                Heading::from((i, 0), Direction::South),
                Heading::from((i, input.max.1), Direction::North),
            ]
        })
        .chain((0..=input.max.1).flat_map(|j| {
            vec![
                Heading::from((0, j), Direction::East),
                Heading::from((input.max.0, j), Direction::West),
            ]
        }))
        .collect();

    candidates
        .par_iter()
        .map(|start| input.clone().explore(start.clone()).len())
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str =r".|...\....
                        |.-.\.....
                        .....|-...
                        ........|.
                        ..........
                        .........\
                        ..../.\\..
                        .-.-/..|..
                        .|....-|.\
                        ..//.|....";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 46);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(TEST)), 51);
    }
}