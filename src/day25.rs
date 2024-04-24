use rand::random;
use rayon::prelude::*;
use std::{
    collections::{HashSet, VecDeque},
    sync::atomic::{AtomicUsize, Ordering},
    // f64::consts::E,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Edge {
    vu: String,
    vv: String,
}

trait Pool {
    fn find_any(&self, v: &String) -> Vec<Edge>;
    fn trim(&mut self, edges: Vec<Edge>);
}

impl Pool for VecDeque<Edge> {
    fn find_any(&self, v: &String) -> Vec<Edge> {
        let mut results = Vec::new();
        for edge in self {
            if edge.vu == *v || edge.vv == *v {
                results.push(edge.clone());
            }
        }
        results
    }

    fn trim(&mut self, edges: Vec<Edge>) {
        self.retain(|edge| !edges.contains(edge));
    }
}

#[derive(Debug, Clone)]
pub struct SuperVertex {
    vertices: Vec<String>,
    edges: Vec<Edge>,
}

impl SuperVertex {
    fn new(vertices: Vec<String>, edges: Vec<Edge>) -> Self {
        SuperVertex {
            vertices,
            edges,
        }
    }

    fn union(self, other: SuperVertex) -> (SuperVertex, Vec<Edge>) {
        let union_vertices = [
            self.vertices,
            other.vertices,
        ].concat();

        let new_edges = [
            self.edges,
            other.edges,
        ].concat();

        let mut duplicates = Vec::new();
        let mut union_edges = Vec::new();

        for edge in new_edges {
            if union_edges.contains(&edge) {
                duplicates.push(edge);
            } else {
                union_edges.push(edge);
            }
        }

        (SuperVertex::new(union_vertices, union_edges), duplicates)
    }
}

trait Graph {
    fn take(&mut self, v: &String) -> SuperVertex;
}

impl Graph for Vec<SuperVertex> {
    fn take(&mut self, v: &String) -> SuperVertex {
        let index = self.iter().position(|sv| sv.vertices.contains(v)).unwrap();
        self.swap_remove(index)
    }
}

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> (Vec<SuperVertex>, VecDeque<Edge>) {
    let mut nodes = HashSet::new();
    let mut edges = VecDeque::new();

    input.lines().for_each(|line| {
        let (start, right) = line.trim().split_once(": ").unwrap();
        nodes.insert(start.to_string());
        right.split(' ').for_each(|end| {
            nodes.insert(end.to_string());
            edges.push_back(Edge { vu: start.to_string(), vv: end.to_string() });
        });
    });

    let svertices = nodes
        .iter()
        .map(|v| SuperVertex {
            vertices: vec![v.clone()],
            edges: edges.find_any(&v),
        })
        .collect();

    (svertices, edges)
}

// Using Karger's algorithm
#[aoc(day25, part1)]
pub fn solve_part1((sverts, edges): &(Vec<SuperVertex>, VecDeque<Edge>)) -> usize {
    // Optimal number of Monte-Carlo simulations
    // let runs = (vertices.len().pow(2) as f64 * (vertices.len() as f64).log(E)) as usize;
    let score = AtomicUsize::new(0);

    (0..1000).into_par_iter().for_each(|_| {
        let mut graph = sverts.clone().to_owned();
        let mut pool = edges.clone().to_owned();

        while graph.len() > 2 {
            let edge = pool.get(random::<usize>() % pool.len()).unwrap();
            let vu = graph.take(&edge.vu);
            let vv = graph.take(&edge.vv);
            let (vw, dup_edges) = vu.union(vv);
            pool.trim(dup_edges);
            graph.push(vw);            
        }
        if pool.len() == 3 {
            let current_score = graph[0].vertices.len() * graph[1].vertices.len();
            score.fetch_max(current_score, Ordering::Relaxed);
            println!("Successful run! Current score = {current_score}");
        } else {
            println!("Failed run.");
        }
    });
    score.load(Ordering::Relaxed)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "jqt: rhn xhk nvd
                        rsh: frs pzl lsr
                        xhk: hfx
                        cmg: qnr nvd lhk bvb
                        rhn: xhk bvb hfx
                        bvb: xhk hfx
                        pzl: lsr hfx nvd
                        qnr: nvd
                        ntq: jqt hfx bvb xhk
                        nvd: lhk
                        lsr: lhk
                        rzs: qnr cmg lsr rsh
                        frs: qnr lhk lsr";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 54);
    }
}