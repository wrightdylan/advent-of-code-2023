type Set = (u32, u32, u32);

pub struct Game {
    pub id: u32, // technically not really needed
    pub sets: Vec<Set>,
}
impl Game {
    pub fn parse(line: &str) -> Game {
        let (id, sets) = line.split_once(':').unwrap();
        let id = id.trim().split(' ').last().unwrap().parse::<u32>().unwrap();
        let sets: Vec<Set> = sets
            .split(';')
            .map(|set| {
                let mut counts: Set = (0, 0, 0);
                set.split(',')
                    .for_each(|count| {
                        let (num, colour) = count.trim().split_once(' ').unwrap();
                        match colour {
                            "red" => counts.0 = num.parse().unwrap(),
                            "green" => counts.1 = num.parse().unwrap(),
                            "blue" => counts.2 = num.parse().unwrap(),
                            _ => {}
                        }
                    });
                counts
            })
            .collect();

        Game { id, sets }
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Game> {
    input.lines().map(|line| Game::parse(line)).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Game]) -> u32 {
    input
        .iter()
        .filter(|game| {
            game.sets.iter().all(|set| set.0 <= 12 && set.1 <= 13 && set.2 <= 14)
        })
        .map(|game| game.id)
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Game]) -> u32 {
    input
        .iter()
        .map(|game| {
            let mut min: Set = game.sets[0];
            for set in &game.sets {
                min.0 = min.0.max(set.0);
                min.1 = min.1.max(set.1);
                min.2 = min.2.max(set.2);
            }
            min.0 * min.1 * min.2
        })
        .sum()
}