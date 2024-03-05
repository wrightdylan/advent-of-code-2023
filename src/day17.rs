#![allow(dead_code)]
use num::integer::Roots;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

type Point = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
    Start,
}

impl Direction {
    fn flip(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East  => Direction::West,
            Direction::West  => Direction::East,
            Direction::Start => Direction::Start,
        }
    }
}

enum Heuristic {
    Chebyshev,
    Euclidean,
    Manhattan,
    Octile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Node {
    pos: Point,
    from: NodeKey,
    f: usize,
    g: usize,
    dir: Direction,
    steps: usize,
}

impl Node {
    fn continues(&self, dir: Direction) -> bool {
        self.dir == dir
    }

    fn from(pos: Point, from: NodeKey, target: Point, g: usize, dir: Direction, steps: usize) -> Node {
        let h = heuristic(&pos, &target, Heuristic::Octile);
        let f = g + h;

        Node { pos, from, f, g, dir, steps }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.cmp(&self.f)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Into<NodeKey> for Node {
    fn into(self) -> NodeKey {
        NodeKey { pos: self.pos, dir: self.dir, steps: self.steps }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeKey {
    pos: Point,
    dir: Direction,
    steps: usize,
}

impl NodeKey {
    fn from(pos: Point, dir: Direction, steps: usize) -> Self {
        NodeKey { pos, dir, steps }
    }
}

trait ClosedList {
    fn initialise(source: Point, target: Point, max: Point) -> Self;
    fn reconstruct_path(&self, last_node: Node) -> Vec<Node>;
}

impl ClosedList for HashMap<NodeKey, Node> {
    fn initialise(source: Point, target: Point, max: Point) -> Self {
        let mut hashmap: HashMap<NodeKey, Node> = HashMap::new();
        for (_, dir) in get_neighbours(source, max) {
            hashmap.insert(
                NodeKey::from(source, dir, 0), 
                Node::from(source, NodeKey::from(source, Direction::Start, 0), target, 0, dir, 0)
            );
        }

        hashmap
    }

    fn reconstruct_path(&self, last_node: Node) -> Vec<Node> {
        let mut path = Vec::new();
        let mut current = last_node;

        while current.from.dir != Direction::Start {
            path.push(current);
            current = self.get(&current.from).expect("Parent not found! Poor orphan.").clone();
        }
        path.push(current);

        path
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct City {
    map: HashMap<Point, usize>,
    max: Point,
}

impl City {
    fn pathfinder(&self, source: Point, target: Point, min_steps: usize, max_steps: usize) -> Option<usize> {
        let mut visited = HashMap::initialise(source, target, self.max);
        let mut priority_queue = BinaryHeap::initialise(&self, source, target.clone());

        while let Some(node) = priority_queue.pop() {
            if node.pos == target && node.steps >= min_steps {
                // self.visualise_path(visited.reconstruct_path(node));
                // self.visualise_visited(visited);
                return Some(node.g);
            }
            if visited.get(&node.into()).is_some_and(|&n| n.g < node.g) {
                continue;
            }
            if !visited.contains_key(&node.into()) {
                visited.insert(node.into(), node);
                for (pos, dir) in get_neighbours(node.pos, self.max) {
                    let next_g = node.g + self.map.get(&pos).unwrap();
                    let steps = if node.continues(dir) { node.steps + 1 } else { 1 };
                    let next = Node::from(pos, NodeKey::from(node.pos, node.dir, node.steps), target, next_g, dir, steps);
                    if next.steps > max_steps || visited.get(&next.into()).is_some_and(|&n| n.g <= next.g) {
                        continue;
                    }
                    if !node.continues(dir) && node.steps < min_steps {
                        continue;
                    }
                    if dir == node.dir.flip() {
                        continue;
                    }
                    priority_queue.push(next);
                }
            }
        }

        None
    }

    fn visualise_path(&self, path: Vec<Node>) {
        let mut city_map: HashMap<Point, String> = self.map.iter()
            .map(|(&point, &value)| (point, value.to_string()))
            .collect();

        for node in &path {
            let symbol = match node.dir {
                Direction::North => "^",
                Direction::East  => ">",
                Direction::South => "v",
                Direction::West  => "<",
                Direction::Start => "#",
            };
            city_map.insert(node.pos, symbol.to_string());
        }

        for row in 0..=self.max.1 {
            let line: String = (0..=self.max.0)
                .map(|col| city_map.get(&(col, row)).cloned().unwrap())
                .collect();
            println!("{}", line);
        }
    }

    fn visualise_visited(&self, visited: HashMap<NodeKey, usize>) {
        let mut city_map: HashMap<Point, String> = self.map.iter()
            .map(|(&point, &value)| (point, value.to_string()))
            .collect();

        for (nodekey, _) in visited {
            city_map.insert(nodekey.pos, "#".to_string());
        }

        for row in 0..=self.max.1 {
            let line: String = (0..=self.max.0)
                .map(|col| city_map.get(&(col, row)).cloned().unwrap())
                .collect();
            println!("{}", line);
        }
    }
}

trait OpenList {
    fn initialise(city: &City, source: Point, target: Point) -> Self;
    fn update_or(&mut self, next: Node) -> Self;
}

impl OpenList for BinaryHeap<Node> {
    // Generalisation to start from anywhere
    fn initialise(city: &City, source: Point, target: Point) -> Self {
        let mut heap = BinaryHeap::<Node>::new();

        for (pos, dir) in get_neighbours(source, city.max) {
            let g = city.map.get(&pos).unwrap().clone();
            heap.push(Node::from(pos, NodeKey::from(source, Direction::Start, 0), target, g, dir, 1));
        }

        heap
    }

    fn update_or(&mut self, next: Node) -> Self {
        let intermediary =self.iter().cloned().collect::<Vec<Node>>();
        if let Some(index) = intermediary.iter().position(|node| node.pos == next.pos) {
            if intermediary[index].g > next.g {
                self.clear();
                self.extend(intermediary);
            }
        } else {
            self.push(next);
        }
        self.to_owned()
    }
}

fn get_neighbours(pos: Point, max: Point) -> Vec<(Point, Direction)> {
    let mut neighbours = Vec::new();

    if pos.1 > 0 {
        neighbours.push(((pos.0, pos.1 - 1), Direction::North));
    }
    if pos.0 < max.0 {
        neighbours.push(((pos.0 + 1, pos.1), Direction::East));
    }
    if pos.1 < max.1 {
        neighbours.push(((pos.0, pos.1 + 1), Direction::South));
    }
    if pos.0 > 0 {
        neighbours.push(((pos.0 - 1, pos.1), Direction::West));
    }

    neighbours
}

// Optional heuristics
fn heuristic(current: &Point, target: &Point, heuristic: Heuristic) -> usize {
    match heuristic {
        Heuristic::Chebyshev => chebyshev(&current, &target),
        Heuristic::Euclidean => euclidean(&current, &target),
        Heuristic::Manhattan => manhattan(&current, &target),
        Heuristic::Octile    => octile(&current, &target),
    }
}

// Chebyshev heuristic estimate
fn chebyshev(current: &Point, target: &Point) -> usize {
    let dx = (current.0 as isize - target.0 as isize).abs() as usize;
    let dy = (current.1 as isize - target.1 as isize).abs() as usize;

    dx.max(dy)
}

// Euclidean heuristic estimate
fn euclidean(current: &Point, target: &Point) -> usize {
    let dx = current.0 as isize - target.0 as isize;
    let dy = current.1 as isize - target.1 as isize;

    (dx.pow(2) + dy.pow(2)).sqrt() as usize
}

// Manhattan heuristic estimate
fn manhattan(current: &Point, target: &Point) -> usize {
    let dx = (current.0 as isize - target.0 as isize).abs() as usize;
    let dy = (current.1 as isize - target.1 as isize).abs() as usize;

    dx + dy
}

// Octile heuristic estimate
fn octile(current: &Point, target: &Point) -> usize {
    let dx = (current.0 as f32 - target.0 as f32).abs();
    let dy = (current.1 as f32 - target.1 as f32).abs();
    
    (1.414 * dx.min(dy) + (dx - dy).abs()).floor() as usize
}

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> City {
    let map: HashMap<Point, usize> = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(move |(col, ch)| {
                    ((col, row), ch.to_string().parse::<usize>().unwrap())
            })
    }).collect();

    let max = map.keys().cloned().max().unwrap_or((0, 0));

    City { map, max }
}

#[aoc(day17, part1)]
pub fn solve_part1(input: &City) -> usize {
    input.pathfinder((0,0), input.max, 1, 3).unwrap_or(0)
}

#[aoc(day17, part2)]
pub fn solve_part2(input: &City) -> usize {
    input.pathfinder((0,0), input.max, 4, 10).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &str= "2413432311323
                        3215453535623
                        3255245654254
                        3446585845452
                        4546657867536
                        1438598798454
                        4457876987766
                        3637877979653
                        4654967986887
                        4564679986453
                        1224686865563
                        2546548887735
                        4322674655533";

    const TEST2: &str= "111111111111
                        999999999991
                        999999999991
                        999999999991
                        999999999991";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST1)), 102);
    }

    #[test]
    fn part2_1_test() {
        assert_eq!(solve_part2(&input_generator(TEST1)), 94);
    }

    #[test]
    fn part2_2_test() {
        assert_eq!(solve_part2(&input_generator(TEST2)), 71);
    }
}
