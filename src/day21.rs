use std::collections::{HashMap, VecDeque};

type Point = (usize, usize);

#[derive(Debug, Clone, PartialEq)]
enum Feature {
    Plot,
    Rock,
    Start,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Garden {
    map: HashMap<Point, Feature>,
    max: Point,
}

impl Garden {
    fn mozaic(&self, steps: usize) -> usize {
        // Assume a square map
        let map_size = self.max.0 + 1;

        // Notes:
        // Width and height = 131 steps (accounting for zero index)
        // From the start point it's 65 steps to the edge of the map, and then
        // another 131 steps to the same edge of the adjacent map.
        // 26_501_365 % steps = 65
        // Interesting to note that the row and column of the start position is empty.
        // The larger picture of the input show a rhomboid void that reaches the edges
        // of the map. This should theoretically simplify tiling calculations, which
        // means this can be solved by algorithm rather than brute force. Alternatively
        // this can also be solved by considering there are two variations of rhomb in
        // alternating patters: one which starts as usual and fills the rhombus, and
        // the inverse which fills the gaps in between. A mosaic can be built from the
        // two types. The downside is that this is a special solution, but I would
        // much prefer to write a general solution.

        // Mosaic version (which doesn't work)
        // let steps_max = self.max.0 / 2;
        // let tiles = self.walk(steps_max, 0);
        // let it = checker_series(steps / map_size);
        // it * tiles.1.len() + (it - 1) * tiles.0.len()
    
        let visited = self.walk_full(steps, 2);

        let y0 = visited
            .values()
            .filter(|dist| **dist <= map_size / 2 && *dist % 2 != 0)
            .count();

        let y1 = visited
            .values()
            .filter(|dist| **dist <= map_size / 2 + map_size && *dist % 2 == 0)
            .count()
            - y0;

        let y2 = visited
            .values()
            .filter(|dist| **dist <= map_size / 2 + map_size * 2 && *dist % 2 != 0)
            .count()
            - y0;

        let (a, b, c) = lpi((y0, y1, y2));
        let n = (steps - (map_size / 2)) / map_size;
        
        a * n * n + b * n + c
    }

    // fn walk(&self, steps: usize) -> usize {
    //     let start_pos = self.map.iter()
    //         .find(|(_, v)| *v == &Feature::Start)
    //         .map(|(key, _)| key)
    //         .unwrap()
    //         .clone();
    //     let mut visited = HashSet::from([start_pos]);
    //     let mut queue = Vec::from(vec![start_pos]);

    //     for step in 0..steps {
    //         let mut frontier: Vec<Point> = Vec::new();
    //         while let Some(pos) = queue.pop() {
    //             for n in get_neighbours(pos, self.max) {
    //                 if visited.contains(&n)
    //                     || frontier.contains(&n)
    //                     || self.map.get(&n).unwrap() == &Feature::Rock {
    //                     continue;
    //                 }
    //                 frontier.push(n);
    //             }
    //         }
    //         if step.is_odd() {
    //             visited.extend(frontier.clone());
    //         }
    //         queue = frontier;
    //     }

    //     visited.len()
    // }

    fn walk_full(&self, steps: usize, depth: usize) -> HashMap<(usize, usize), usize> {
        // Assume a square map
        let map_size = self.max.0 + 1;
        let start_pos = self.map.iter()
            .find(|(_, v)| *v == &Feature::Start)
            .map(|(key, _)| (key.0 + (map_size) * depth, key.1 + (map_size) * depth))
            .unwrap()
            .clone();
        let mut visited = HashMap::from([(start_pos, 0)]);
        let mut queue = VecDeque::from(vec![(start_pos, 0)]);

        while let Some((pos, dist)) = queue.pop_front() {
            for n in get_neighbours(pos, (map_size * (2 * depth + 1), map_size * (2 * depth + 1))) {
                if visited.contains_key(&n)
                    || dist >= steps
                    || self.map.get(&( n.0 % map_size, n.1 % map_size )).unwrap() == &Feature::Rock {
                    continue;
                }
                visited.insert(n, dist + 1);
                queue.push_back((n, dist + 1));
            }
        }
            
        visited
    }
}

fn get_neighbours(pos: Point, max: Point) -> Vec<Point> {
    let mut neighbours = Vec::new();

    if pos.1 > 0 {
        neighbours.push((pos.0, pos.1 - 1));
    }
    if pos.0 < max.0 {
        neighbours.push((pos.0 + 1, pos.1));
    }
    if pos.1 < max.1 {
        neighbours.push((pos.0, pos.1 + 1));
    }
    if pos.0 > 0 {
        neighbours.push((pos.0 - 1, pos.1));
    }

    neighbours
}

// Lagrange Polynomial Interpolator
fn lpi(values: (usize, usize, usize)) -> (usize, usize, usize) {
    let a = (values.2 - 2 * values.1) / 2;
    let b = values.1 - a;
    let c = values.0;

    (a, b, c)
}

// Black squares on a checkerboard (assuming start is black)
// fn checker_series(n: usize) -> usize {
//     (2 * n).pow(2)/2 + (2 * n + 1)
// }

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> Garden {
    let map: HashMap<Point, Feature> = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(move |(col, ch)| {
                    ((col, row), match ch {
                        'S' => Feature::Start,
                        '.' => Feature::Plot,
                        '#' => Feature::Rock,
                        _   => panic!("Unrecognised tile type: {}", ch),
                    })
            })
    }).collect();

    let max = map.keys().cloned().max().unwrap_or((0, 0));

    Garden { map, max }
}

#[aoc(day21, part1)]
pub fn solve_part1(input: &Garden) -> usize {
    // input.walk(64)
    input.walk_full(64, 0)
        .values()
        .filter(|dist| **dist <= 131 / 2 && *dist % 2 == 0)
        .count()
}

#[aoc(day21, part2)]
pub fn solve_part2(input: &Garden) -> usize {
    input.mozaic(26_501_365)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     const TEST: &str = "...........
//                         .....###.#.
//                         .###.##..#.
//                         ..#.#...#..
//                         ....#.#....
//                         .##..S####.
//                         .##..#...#.
//                         .......##..
//                         .##.#.####.
//                         .##..##.##.
//                         ...........";

//     fn generate_map() -> Garden {
//         input_generator(TEST)
//     }

//     #[test]
//     fn part1_test() {
//         let garden = generate_map();
//         assert_eq!(garden.walk(6), 16);
//     }
// }