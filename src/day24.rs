use z3::ast::{Ast, Int};
use z3::{Config, Context, Solver};

type Point = (i64, i64);

// const TEST_SIZE: Point = (7, 27);
const INPUT_SIZE: Point = (200_000_000_000_000, 400_000_000_000_000);

#[derive(Debug)]
struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

#[derive(Debug)]
struct Line {
    a: f64,
    b: f64,
    c: f64,
}

impl Line {
    fn determinant(&self, other: &Line) -> f64 {
        self.a * other.b - self.b * other.a
    }
}

impl From<Vec<i64>> for Vec3<i64> {
    fn from(values: Vec<i64>) -> Self {
        if values.len() != 3 {
            panic!("Invalid input format.");
        }

        let [x, y, z] = {
            let mut iter = values.into_iter();
            [iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()]
        };

        Vec3 { x, y, z }
    }
}

#[derive(Debug)]
pub struct Hailstone<T> {
    pos: Vec3<T>,
    vel: Vec3<T>,
}

impl Hailstone<f64> {
    // Converts from parametric form to standard form
    fn to_std(&self) -> Line {
        Line { a: self.vel.y, b: -self.vel.x, c: (self.vel.y * self.pos.x - self.vel.x * self.pos.y) }
    }
}

impl Hailstone<i64> {
    fn recast(&self) -> Hailstone<f64> {
        Hailstone {
            pos: Vec3 { x: self.pos.x as f64, y: self.pos.y as f64, z: self.pos.z as f64 },
            vel: Vec3 { x: self.vel.x as f64, y: self.vel.y as f64, z: self.vel.z as f64 },
        }
    }
}

fn int_in_box(a: &Hailstone<i64>, b: &Hailstone<i64>, bounds: &Point) -> Option<(f64, f64)> {
    let (ha, hb) = (a.recast(), b.recast());
    let (la, lb) = (ha.to_std(), hb.to_std());

    let det = la.determinant(&lb);

    if det == 0.0 {
        return None;
    }

    let x = (la.c * lb.b - lb.c * la.b) / det;
    let y = (lb.c * la.a - la.c * lb.a) / det;

    if ((x - ha.pos.x)/ha.vel.x > 0.0 ) && ((x - hb.pos.x)/hb.vel.x > 0.0 )
    && (bounds.0 as f64..=bounds.1 as f64).contains(&x)
    && (bounds.0 as f64..=bounds.1 as f64).contains(&y) {
        return Some((x, y));
    }
    
    None
}

fn solve(hs: &Vec<Hailstone<i64>>) -> i64 {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let (px, py, pz, vx, vy, vz) = (
        Int::new_const(&ctx, "px"),
        Int::new_const(&ctx, "py"),
        Int::new_const(&ctx, "pz"),
        Int::new_const(&ctx, "vx"),
        Int::new_const(&ctx, "vy"),
        Int::new_const(&ctx, "vz"),
    );

    for hailstone in hs {
        let (pxn, pyn, pzn, vxn, vyn, vzn) = (
            Int::from_i64(&ctx, hailstone.pos.x),
            Int::from_i64(&ctx, hailstone.pos.y),
            Int::from_i64(&ctx, hailstone.pos.z),
            Int::from_i64(&ctx, hailstone.vel.x),
            Int::from_i64(&ctx, hailstone.vel.y),
            Int::from_i64(&ctx, hailstone.vel.z),
        );
        let tn = Int::fresh_const(&ctx, "t");

        solver.assert(&(&pxn + &vxn * &tn)._eq(&(&px + &vx * &tn)));
        solver.assert(&(&pyn + &vyn * &tn)._eq(&(&py + &vy * &tn)));
        solver.assert(&(&pzn + &vzn * &tn)._eq(&(&pz + &vz * &tn)));
    }

    solver.check();
    let model = solver.get_model().unwrap();
    let (x, y, z) = (
        model.get_const_interp(&px).unwrap().as_i64().unwrap(),
        model.get_const_interp(&py).unwrap().as_i64().unwrap(),
        model.get_const_interp(&pz).unwrap().as_i64().unwrap(),
    );

    x + y + z
}

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Vec<Hailstone<i64>> {
    input
        .lines()
        .map(|line| {
            let (position, velocity) = line.trim().split_once(" @ ").unwrap();
            let pos: Vec3<i64> = position
                .split(", ")
                .map(|num| num.trim().parse().expect("Invalid position format."))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let vel: Vec3<i64> = velocity
                .split(", ")
                .map(|num| num.trim().parse().expect("Invalid velocity format."))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            Hailstone { pos, vel }
        })
        .collect()
}

#[aoc(day24, part1)]
pub fn solve_part1(input: &Vec<Hailstone<i64>>) -> usize {
    input
        .iter()
        .enumerate()
        .flat_map(|(i, ha)| input.iter().skip(i + 1).map(move |hb| (ha, hb)))
        .filter(|(ha, hb)| int_in_box(ha, hb, &INPUT_SIZE).is_some())
        .count()
}

// Part 2 is effectively a ray-tracer, but Z3 SMT solver seems to be a popular solution.
#[aoc(day24, part2)]
pub fn solve_part2(input: &Vec<Hailstone<i64>>) -> i64 {
    solve(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "19, 13, 30 @ -2,  1, -2
                        18, 19, 22 @ -1, -1, -2
                        20, 25, 34 @ -2, -2, -4
                        12, 31, 28 @ -1, -2, -1
                        20, 19, 15 @  1, -5, -3";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 2);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(TEST)), 47);
    }
}