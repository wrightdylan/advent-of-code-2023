use std::collections::HashMap;

type Workflows = HashMap<String, Vec<Rule>>;

#[derive(Debug)]
pub struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn get(&self, field: &Category) -> usize {
        match field {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
    }

    fn sum_values(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug)]
pub enum Rule {
    Accepted,
    Rejected,
    Advance(String),
    GT(Category, usize, String),
    LT(Category, usize, String),
}

#[derive(Debug)]
pub enum Category {
    X,
    M,
    A,
    S
}

#[derive(Debug, Clone, Copy)]
pub struct NumRange {
    min: usize,
    max: usize,
}

impl NumRange {
    fn default() -> Self {
        NumRange { min: 1, max: 4000 }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Parts {
    x: NumRange,
    m: NumRange,
    a: NumRange,
    s: NumRange,
}

impl Parts {
    fn combinations(&self) -> usize {
        (self.x.max - self.x.min + 1) *
        (self.m.max - self.m.min + 1) *
        (self.a.max - self.a.min + 1) *
        (self.s.max - self.s.min + 1)
    }

    fn initialise() -> Self {
        Parts {
            x: NumRange::default(),
            m: NumRange::default(),
            a: NumRange::default(),
            s: NumRange::default(),
        }
    }

    fn update_max(&self, field: &Category, max: usize) -> Self {
        match field {
            Category::X => {
                Parts { x: NumRange { min: self.x.min, max }, m: self.m, a: self.a, s: self.s }
            },
            Category::M => {
                Parts { x: self.x, m: NumRange { min: self.m.min, max }, a: self.a, s: self.s }
            },
            Category::A => {
                Parts { x: self.x, m: self.m, a: NumRange { min: self.a.min, max }, s: self.s }
            },
            Category::S => {
                Parts { x: self.x, m: self.m, a: self.a, s: NumRange { min: self.s.min, max } }
            },
        }
    }

    fn update_min(&self, field: &Category, min: usize) -> Self {
        match field {
            Category::X => {
                Parts { x: NumRange { min, max: self.x.max }, m: self.m, a: self.a, s: self.s }
            },
            Category::M => {
                Parts { x: self.x, m: NumRange { min, max: self.m.max }, a: self.a, s: self.s }
            },
            Category::A => {
                Parts { x: self.x, m: self.m, a: NumRange { min, max: self.a.max }, s: self.s }
            },
            Category::S => {
                Parts { x: self.x, m: self.m, a: self.a, s: NumRange { min, max: self.s.max } }
            },
        }
    }
}

fn categorise(field: &str) -> Category {
    match field {
        "x" => Category::X,
        "m" => Category::M,
        "a" => Category::A,
        "s" => Category::S,
        _   => panic!("Not a valid category."),
    }
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> (Workflows, Vec<Part>) {
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    
    (
        workflows
            .lines()
            .map(|line| {
                let (key, rule_str) = line.trim_matches('}').split_once('{').unwrap();
                let rules = rule_str.split(',').map(|segment| {
                    match segment {
                        "A" => Rule::Accepted,
                        "R" => Rule::Rejected,
                        _   => {
                            if !segment.contains(':') {
                                return Rule::Advance(segment.to_string());
                            }

                            let (rule, to) = segment.split_once(':').unwrap();
                            if rule.contains('>') {
                                let (field, value) = rule.split_once('>').unwrap();
                                Rule::GT(categorise(field), value.parse().unwrap(), to.to_string())
                            } else if rule.contains('<') {
                                let (field, value) = rule.split_once('<').unwrap();
                                Rule::LT(categorise(field), value.parse().unwrap(), to.to_string())
                            } else {
                                panic!("Unknown rule.");
                            }
                        }
                    }
                })
                .collect();
                (key.to_string(), rules)
            })
            .collect(),
        parts
            .split('\n')
            .map(|line| {
                let mut part = Part { x: 0, m: 0, a: 0, s: 0 };
                for pair in line.trim_matches(|c| c == '{' || c == '}').split(',') {
                    let (key, value) = pair.split_once('=').unwrap();
                    match key {
                        "x" => part.x = value.parse().unwrap(),
                        "m" => part.m = value.parse().unwrap(),
                        "a" => part.a = value.parse().unwrap(),
                        "s" => part.s = value.parse().unwrap(),
                        _   => panic!("Not avalid input."),
                    }
                }
                part
            })
            .collect()
    )
}

#[aoc(day19, part1)]
pub fn solve_part1((workflows, parts): &(Workflows, Vec<Part>)) -> usize {
    let mut accepted = Vec::new();

    'parts: for part in parts {
        let mut work_id = "in";

        'workflows: loop {
            if work_id == "A" {
                accepted.push(part);
                continue 'parts;
            } else if work_id == "R" {
                continue 'parts;
            }
            let rules = workflows.get(work_id).unwrap();
            for rule in rules {
                match rule {
                    Rule::Accepted => {
                        accepted.push(part);
                        continue 'parts;
                    },
                    Rule::Rejected => continue 'parts,
                    Rule::Advance(id) => {
                        work_id = id.as_str();
                        continue 'workflows;
                    },
                    Rule::GT(field, value, result) => {
                        if part.get(field) > *value {
                            work_id = result;
                            continue 'workflows;
                        }
                    },
                    Rule::LT(field, value, result) => {
                        if part.get(field) < *value {
                            work_id = result;
                            continue 'workflows;
                        }
                    }
                }
            }
            
        }
    }

    accepted.iter().map(|part| part.sum_values()).sum()
}

// This bitch is another one which will take eons to brute force...
#[aoc(day19, part2)]
pub fn solve_part2((workflows, _): &(Workflows, Vec<Part>)) -> usize {
    let mut queue: Vec<(Parts, &str)> = vec![(Parts::initialise(), "in")];
    let mut accepted: Vec<Parts> = Vec::new();

    while let Some(range) = queue.pop() {
        let (mut parts, work_id) = range;
        if work_id == "A" {
            accepted.push(parts);
            continue;
        } else if work_id == "R" {
            continue;
        }

        let rules = workflows.get(work_id).unwrap();
        for rule in rules {
            match rule {
                Rule::Accepted => {
                    accepted.push(parts);
                    continue;
                },
                Rule::Rejected => {
                    continue;
                },
                Rule::Advance(id) => {
                    queue.push((parts, id.as_str()));
                    continue;
                },
                Rule::GT(field, value, result) => {
                    queue.push((parts.update_min(field, value + 1), result.as_str()));
                    parts = parts.update_max(field, *value);
                },
                Rule::LT(field, value, result) => {
                    queue.push((parts.update_max(field, value - 1), result.as_str()));
                    parts = parts.update_min(field, *value);
                },
            }
        }
    }

    accepted.iter().map(|parts| parts.combinations()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 19_114);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(TEST)), 167_409_079_868_000);
    }
}