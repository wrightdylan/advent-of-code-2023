use std::collections::{HashMap, HashSet, VecDeque};

type Point = (usize, usize);

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn flip(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East  => Direction::West,
            Direction::West  => Direction::East,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Forest,
    Path,
    Slope(Direction),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Forest {
    map: HashMap<Point, Tile>,
    max: Point,
    start: Point,
    end: Point,
}

impl Forest {
    fn get_neighbours(&self, pos: Point) -> Vec<(Point, Direction)> {
        let mut neighbours = Vec::new();
    
        if pos.1 < self.max.1 {
            neighbours.push(((pos.0, pos.1 + 1), Direction::South));
        }
        if pos.0 < self.max.0 {
            neighbours.push(((pos.0 + 1, pos.1), Direction::East));
        }
        if pos.1 > 0 {
            neighbours.push(((pos.0, pos.1 - 1), Direction::North));
        }
        if pos.0 > 0 {
            neighbours.push(((pos.0 - 1, pos.1), Direction::West));
        }
    
        neighbours
    }
}

#[derive(Debug)]
pub struct Edge {
    start: Option<usize>,
    end: Option<usize>,
    weight: usize,
}

#[derive(Debug)]
pub struct Node {
    pos: Point,
    bp: Vec<usize>,
    fp: Vec<usize>,
}

impl Node {
    fn new(pos: Point) -> Self {
        Node { pos, bp: Vec::new(), fp: Vec::new() }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Status {
    Edge,
    Node,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Vertex {
    f: usize,
    g: usize,
    inputs: Vec<usize>,
    from: Option<usize>,
    closed: bool,
}

impl Vertex {
    fn default() -> Self {
        Vertex { f: 0, g: 0, inputs: vec![], from: None, closed: false }
    }
}

trait Traverse {
    fn step_forward(&self, dir: &Direction) -> Point;
    fn step_backward(&self, dir: &Direction) -> Point;
}

impl Traverse for Point {
    fn step_forward(&self, dir: &Direction) -> Point {
        match dir {
            Direction::North => (self.0, self.1 - 1),
            Direction::East  => (self.0 + 1, self.1),
            Direction::South => (self.0, self.1 + 1),
            Direction::West  => (self.0 - 1, self.1),
        }
    }

    fn step_backward(&self, dir: &Direction) -> Point {
        self.step_forward(&dir.flip())
    }
}

#[derive(Debug)]
enum Propagation {
    Forward,
    Backward,
}

#[derive(Debug)]
pub struct SawState {
    vertex: usize,
    cost: usize,
    visited: Vec<usize>,
}

impl SawState {
    fn next_node(&self, next: usize, prop: Propagation, edges: &HashMap<String, Edge>) -> Option<SawState> {
        if !self.visited.contains(&next) {
            let cost = match prop {
                Propagation::Forward => edges.get(&format!("{}To{}", self.vertex, next)).unwrap().weight + 1 + self.cost,
                Propagation::Backward => edges.get(&format!("{}To{}", next, self.vertex)).unwrap().weight + 1 + self.cost,
            };
            let mut visited = self.visited.clone();
            visited.push(self.vertex);
            return Some(SawState { vertex: next, cost, visited })
        }
        None
    }

    fn start(start_weight: usize) -> Self {
        SawState { vertex: 0, cost: start_weight + 1, visited: vec![] }
    }
}

fn find_node(node_list: &HashMap<usize, Node>, pos: &Point) -> Option<usize> {
    for (key, node) in node_list {
        if &node.pos == pos {
            return Some(*key);
        }
    }
    None
}

fn graph_abstraction(forest: &Forest) -> (HashMap<usize, Node>, HashMap<String, Edge>) {
    let mut node_list = HashMap::new();
    let mut edge_list = HashMap::new();
    let mut queue = Vec::from([forest.start]);
    let mut visited = HashSet::new();
    let mut segment = HashSet::new();
    let mut status = Status::Edge;
    let mut node_num = 0;
    let mut start_node = 0;

    while let Some(pos) = queue.pop() {
        if status == Status::Node && !visited.contains(&pos) {
            node_list.insert(node_num, Node::new(pos));
            visited.insert(pos);
        } else {
            segment.insert(pos);
        }

        let neighbours = forest.get_neighbours(pos);

        for n in neighbours {
            match forest.map.get(&n.0).unwrap().clone() {
                Tile::Forest => continue,
                Tile::Path => if !segment.contains(&n.0) && !visited.contains(&n.0) {
                        queue.push(n.0);
                    },
                Tile::Slope(_) => {
                    if status == Status::Edge && segment.len() > 1 && !segment.contains(&n.0) && !visited.contains(&n.0) {
                        queue.push(n.0);
                    } else if status == Status::Node {
                        match n.1 {
                            Direction::South | Direction::East => {
                                if !segment.contains(&n.0) && !visited.contains(&n.0) {
                                    queue.push(n.0);
                                }
                            },
                            Direction::North | Direction::West => continue,
                        }
                    }
                },
            }
        }

        if status == Status::Node && segment.len() == 0 {
            status = Status::Edge;
            node_num += 1;
        }
        if segment.contains(&forest.end) {
            edge_list.insert(
                "End".to_string(),
                Edge { start: Some(start_node), end: None, weight: segment.len() }
            );
            visited.extend(segment.clone().into_iter());
            segment.clear();
        }

        match forest.map.get(&pos).unwrap().clone() {
            Tile::Slope(dir) => {
                if segment.contains(&forest.start) {
                    edge_list.insert(
                        "Start".to_string(),
                        Edge { start: None, end: Some(0), weight: segment.len() - 1 }
                    );
                    visited.extend(segment.clone().into_iter());
                    segment.clear();
                    status = Status::Node;
                } else if segment.len() == 1 {
                    start_node = find_node(&node_list, &pos.step_backward(&dir)).unwrap();
                } else if segment.len() > 1 {
                    let end_node = find_node(&node_list, &pos.step_forward(&dir)).unwrap_or(node_num);
                    edge_list.insert(
                        format!("{start_node}To{end_node}"),
                        Edge { start: Some(start_node), end: Some(end_node), weight: segment.len() }
                    );
                    visited.extend(segment.clone().into_iter());
                    segment.clear();
                    if !visited.contains(&pos.step_forward(&dir)) {
                        status = Status::Node;
                    }
                }
            },
            _ => continue,
        }
    }

    (node_list, edge_list)
}

// fn manhattan(current: &Point, target: &Point) -> usize {
//     let dx = (current.0 as isize - target.0 as isize).abs() as usize;
//     let dy = (current.1 as isize - target.1 as isize).abs() as usize;

//     dx + dy
// }

fn parse_input(input: &str) -> Forest {
    let map: HashMap<Point, Tile> = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(move |(col, ch)| {
                    ((col, row), match ch {
                        '#' => Tile::Forest,
                        '.' => Tile::Path,
                        '^' => Tile::Slope(Direction::North),
                        '>' => Tile::Slope(Direction::East),
                        'v' => Tile::Slope(Direction::South),
                        '<' => Tile::Slope(Direction::West),
                        _   => panic!("Unrecognised tile type: {}", ch),
                    })
            })
    }).collect();

    let max = map.keys().cloned().max().unwrap_or((0, 0));

    Forest { map, max, start: (1, 0), end: (max.0 - 1, max.1) }
}

fn parse_edges(edge_list: &HashMap<String, Edge>) -> Vec<Point> {
    edge_list
        .keys()
        .filter_map(|key| {
            key.split_once("To").and_then(|(start, end)| {
                start.parse::<usize>().ok().and_then(|start| {
                    end.parse::<usize>().ok().map(|end| (start, end))
                })
            })
        })
        .collect()
}

// Note the slopes act like a check valve, and also that there are no slopes heading west or north.
// That is to say there are distinct node points, and travel is limited along east and south lines
// along node points. At least this is the case for part 1, but for part 2 travel can be bidirectional.

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> (HashMap<usize, Node>, HashMap<String, Edge>) {
    let forest = parse_input(input);
    let (mut node_list, edge_list) = graph_abstraction(&forest);
    
    for (start, end) in parse_edges(&edge_list) {
        node_list.get_mut(&start).unwrap().fp.push(end);
        node_list.get_mut(&end).unwrap().bp.push(start);
    }

    (node_list, edge_list)
}

#[allow(unused_assignments)]
#[aoc(day23, part1)]
pub fn solve_part1((nodes, edges): &(HashMap<usize, Node>, HashMap<String, Edge>)) -> usize {
    let mut queue = VecDeque::from(["Start".to_string()]);
    let target = edges.get(&"End".to_string()).unwrap().start.unwrap();
    let mut vertices = (0..nodes.len()).map(|i| (i, Vertex::default())).collect::<HashMap<usize, Vertex>>();
    let mut new_weight = 0;

    while let Some(edge_id) = queue.pop_front() {
        let edge = edges.get(&edge_id).unwrap();
        if let Some(edge_start) = edge.start {
            new_weight = vertices.get(&edge_start).unwrap().g + edge.weight + 1;
        } else {
            new_weight = edge.weight + 1;
        }
        let edge_end = edge.end.unwrap();
        let vertex = vertices.get_mut(&edge_end).unwrap();
        if edge_end == 0 {
            vertex.closed = true;
            vertex.g = edge.weight + 1;
        } else {
            vertex.inputs.push(edge_end);
            let edge_start = edge.start.unwrap();
            if new_weight > vertex.g {
                vertex.g = new_weight;
                vertex.from = Some(edge_start);
            }
            if vertex.inputs.len() == nodes.get(&edge_end).unwrap().bp.len() {
                vertex.closed = true;
            }
        }
        if vertex.closed && edge_end != edges.get(&"End".to_string()).unwrap().start.unwrap() {
            for next_node in nodes.get(&edge_end).unwrap().fp.iter() {
                queue.push_back(format!("{edge_end}To{next_node}"));
            }
        }
    }

    vertices.get(&target).unwrap().g + edges.get(&"End".to_string()).unwrap().weight
}

// A fully undirected walk can result in dead ends, i.e. when looping back on a
// perimeter. This then needs to be a self-avoiding rook walk, so we can assume
// that nodes on the perimeter must be directed, and nodes within the centre of
// the map are undirected.
// May be useful: https://arxiv.org/abs/2107.11542

#[aoc(day23, part2)]
pub fn solve_part2((nodes, edges): &(HashMap<usize, Node>, HashMap<String, Edge>)) -> usize {
    let mut stack = Vec::from([SawState::start(edges.get(&"Start".to_string()).unwrap().weight)]);
    let target = edges.get(&"End".to_string()).unwrap().start.unwrap();
    let mut scores: Vec<usize> = Vec::new();

    while let Some(walk) = stack.pop() {
        if walk.vertex == target {
            scores.push(walk.cost + edges.get(&"End".to_string()).unwrap().weight);
        } else {
            let fps = nodes.get(&walk.vertex).unwrap().fp.clone();
            let bps = nodes.get(&walk.vertex).unwrap().bp.clone();
            if fps.len() + bps.len() < 4 {
                stack.extend(fps.iter().filter_map(|node| walk.next_node(*node, Propagation::Forward, &edges)));
                if bps.len() == 2 {
                    for back_node in bps {
                        if let Some(node) = nodes.get(&back_node) {
                            if node.fp.len() + node.bp.len() == 4 {
                                if let Some(next_node) = walk.next_node(back_node, Propagation::Backward, &edges) {
                                    stack.push(next_node);
                                }
                            }
                        }
                    }
                }
            } else {
                stack.extend(fps.iter().filter_map(|node| walk.next_node(*node, Propagation::Forward, &edges)));
                stack.extend(bps.iter().filter_map(|node| walk.next_node(*node, Propagation::Backward, &edges)));
            }
        }
    }

    scores.push(0);
    scores.iter().max().unwrap().clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "#.#####################
                        #.......#########...###
                        #######.#########.#.###
                        ###.....#.>.>.###.#.###
                        ###v#####.#v#.###.#.###
                        ###.>...#.#.#.....#...#
                        ###v###.#.#.#########.#
                        ###...#.#.#.......#...#
                        #####.#.#.#######.#.###
                        #.....#.#.#.......#...#
                        #.#####.#.#.#########v#
                        #.#...#...#...###...>.#
                        #.#.#v#######v###.###v#
                        #...#.>.#...>.>.#.###.#
                        #####v#.#.###v#.#.###.#
                        #.....#...#...#.#.#...#
                        #.#########.###.#.#.###
                        #...###...#...#...#.###
                        ###.###.#.###v#####v###
                        #...#...#.#.>.>.#.>.###
                        #.###.###.#.###.#.#v###
                        #.....###...###...#...#
                        #####################.#";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 94);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(TEST)), 154);
    }
}