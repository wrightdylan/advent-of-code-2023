use std::collections::{HashSet, VecDeque};

use Colinear::*;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Colinear {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    fn from_str(point_str: &str) -> Self {
        let vec: Vec<usize> = point_str
            .split(',')
            .map(|num| num.parse().unwrap())
            .collect();

        Point {
            x: vec[0],
            y: vec[1],
            z: vec[2],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Brick {
    head: Point,
    tail: Point,
    axis: Colinear,
    dominates: Vec<usize>,
    dominated: Vec<usize>,
}

impl Brick {
    fn extend_dominated(&mut self, doms: &Vec<(usize, Option<usize>)>) {
        for (_, id) in doms {
            if let Some(brick_id) = id {
                self.dominated.push(*brick_id);
            }
        }
    }

    fn height(&self) -> usize {
        match self.axis {
            Z => self.tail.z - self.head.z + 1,
            _ => 1,
        }
    }

    fn pearls(&self) -> Vec<(usize, usize)> {
        match self.axis {
            X => (self.head.x..=self.tail.x).map(|x| (x, self.head.y)).collect(),
            Y => (self.head.y..=self.tail.y).map(|y| (self.head.x, y)).collect(),
            Z => vec![(self.head.x, self.head.y)],
        }
    }
}

trait Tetris {
    fn disintegrate(&self, id: usize) -> usize;
    fn import(&mut self, sups: Vec<Vec<usize>>);
}

impl Tetris for Vec<Brick> {
    fn disintegrate(&self, id: usize) -> usize {
        let mut count = 0;
        let mut queue = VecDeque::from([id]);
        let mut disintegrated = HashSet::new();

        while let Some(brick_id) = queue.pop_front() {
            disintegrated.insert(brick_id);
            for &dom_id in &self[brick_id].dominates {
                if self[dom_id].dominated.iter().all(|&id| disintegrated.contains(&id)) {
                    queue.push_back(dom_id);
                    count += 1;
                }
            }
        }

        count
    }

    fn import(&mut self, sups: Vec<Vec<usize>>) {
        for (id, supports) in sups.iter().enumerate() {
            self[id].dominates.extend(supports);
        }
    }
}

trait HeightRecord {
    fn drop(&mut self, brick: &Brick, id: usize, doms: &Vec<(usize, Option<usize>)>);
    fn find_doms(&self, brick: &Brick) -> Vec<(usize, Option<usize>)>;
}

impl HeightRecord for [(usize, Option<usize>); 100] {
    fn drop(&mut self, brick: &Brick, id: usize, doms: &Vec<(usize, Option<usize>)>) {
        let points = brick.pearls();
        let new_z = doms.first().cloned().unwrap().0 + brick.height();
        for (x, y) in points.iter() {
            self[10 * y + x] = (new_z, Some(id));
        }
    }

    fn find_doms(&self, brick: &Brick) -> Vec<(usize, Option<usize>)> {
        match brick.axis {
            Z => {
                vec![self[10 * brick.head.y + brick.head.x]]
            },
            _ => {
                let tops = brick.pearls().iter().map(|(x, y)| self[10 * y + x]).collect::<Vec<_>>();
                let max = tops.iter().map(|&(top, _)| top).max().unwrap();
                let uniques: HashSet<(usize, Option<usize>)> = tops.into_iter().filter(|&(top, _)| top == max).collect();
                uniques.iter().map(|(top, opt)| (*top, *opt)).collect()
            },
        }
    }
}

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Vec<Brick> {
    let mut bricks = input
        .lines()
        .map(|line| {
            let (head, tail) = line
                .trim()
                .split_once('~')
                .map(|(l, r)| (Point::from_str(l), Point::from_str(r)))
                .expect("Invalid input format.");
            let axis = match (head.x != tail.x, head.y != tail.y) {
                (true, _) => X,
                (_, true) => Y,
                _ => Z,
            };
            Brick { head, tail, axis, dominates: vec![], dominated: vec![] }
        })
        .collect::<Vec<Brick>>();

    bricks.sort_by_key(|brick| brick.head.z);

    // Bricks are all confined to a finite x,y grid of infinite height
    // The actions performed is more like Tetris, but the result resembles a Jenga tower
    let mut jenga: [(usize, Option<usize>); 100] = [(0, None); 100];
    let mut dominates = vec![vec![]; bricks.len()];

    for (id, brick) in bricks.iter_mut().enumerate() {
        let doms = jenga.find_doms(brick);
        brick.extend_dominated(&doms);
        doms.iter()
            .filter_map(|(_, dom_id)| dom_id.clone())
            .for_each(|brick_id| dominates[brick_id].push(id));
        jenga.drop(brick, id, &doms);
    }
    bricks.import(dominates);

    bricks
}

#[aoc(day22, part1)]
pub fn solve_part1(input: &Vec<Brick>) -> usize {
    input
        .iter()
        .filter(|brick| {
            brick.dominates
                .iter()
                .all(|&id| input[id].dominated.len() > 1)
        })
        .count()
}

#[aoc(day22, part2)]
pub fn solve_part2(input: &Vec<Brick>) -> usize {
    let antisafe: Vec<usize> = input
        .iter()
        .enumerate()
        .filter_map(|(id, brick)| {
            if brick.dominates
                .iter()
                .all(|&dom_id| input[dom_id].dominated.len() > 1){
                    None
                } else {
                    Some(id)
                }
        })
        .collect();

    antisafe.iter().map(|brick| input.disintegrate(brick.clone())).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "1,0,1~1,2,1
                        0,0,2~2,0,2
                        0,2,3~2,2,3
                        0,0,4~0,2,4
                        2,0,5~2,2,5
                        0,1,6~2,1,6
                        1,1,8~1,1,9";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 5);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(TEST)), 7);
    }
}