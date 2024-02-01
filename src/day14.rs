use core::fmt;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    Cube,
    Round,
    Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Platform {
    map: HashMap<(usize, usize), Tile>,
    width: usize,
    height: usize,
}

#[allow(dead_code)]
impl Platform {
    fn calculate_load(&self) -> usize {
        self.map
            .iter()
            .filter(|&(_, &value)| value == Tile::Round)
            .map(|((_, row), _)| self.height - row)
            .sum()
    }

    fn cycle(self) -> Self {
        self.tip_north().tip_west().tip_south().tip_east()
    }

    fn cycles(self, n: usize) -> Self {
        (0..n).fold(self, |acc, _| acc.cycle())
    }

    // T&H algorithms were mentioned in the solutions megathread as a means of cycle detection
    fn floyd_cycles(self, n: usize) -> Self {
        let mut tortoise = self.clone().cycle();
        let mut hare = self.clone().cycles(2);
        while tortoise != hare {
            tortoise = tortoise.cycle();
            hare = hare.cycles(2);
        }
        
        // Find the position µ of first repitition
        let mut mu = 0;
        tortoise = self.clone();
        while tortoise != hare {
            tortoise = tortoise.cycle();
            hare = hare.cycle();
            mu += 1;
        }

        // Find the shortest cycle length λ
        let mut lambda = 1;
        hare = tortoise.clone().cycle();
        while tortoise != hare {
            hare = hare.cycle();
            lambda += 1;
        }

        // Repeat the cycle n % cycle_length times
        // µ = 163, λ = 18, modulo = 9, λ skips = 55,555,555, total cycles skipped = 999,999,990
        self.cycles(mu).cycles((n - mu) % lambda)
    }

    // Alternative method of T&H cycle detection
    fn brent_cycles(self, n: usize) -> Self {
        let mut power = 1;
        let mut lambda = 1;
        let mut tortoise = self.clone();
        let mut hare = self.clone().cycle();

        // Main phase using powers of deux
        while tortoise != hare {
            if power == lambda {
                tortoise = hare.clone();
                power *= 2;
                lambda = 0;
            }
            hare = hare.cycle();
            lambda += 1;
        }

        // Find the position of the first repetition
        tortoise = self.clone();
        hare = self.clone().cycles(lambda);

        // Tortoise and hare move at the same speed until they agree
        let mut mu = 0;
        while tortoise != hare {
            tortoise = tortoise.cycle();
            hare = hare.cycle();
            mu += 1;
        }

        self.cycles(mu).cycles((n - mu) % lambda)
    }

    fn tip_east(mut self) -> Self {
        for col in (0..self.width - 1).rev() {
            for row in 0..self.height {
                if let Some(&Tile::Round) = self.map.get(&(col, row)) {
                    let mut last_empty_col = None;
                    for search_col in col + 1..self.width {
                        if let Some(&Tile::Empty) = self.map.get(&(search_col, row)) {
                            last_empty_col = Some(search_col);
                        } else {
                            break;
                        }
                    }
                    if let Some(last_empty_col) = last_empty_col {
                        self.map.insert((col, row), Tile::Empty);
                        self.map.insert((last_empty_col, row), Tile::Round);
                    }
                }
            }
        }

        self
    }

    // This reqires multiple loops as it moves round rocks only 1 position each iteration.
    // fn tip_north(mut self) -> Platform {
    //     loop {
    //         let mut differences = 0;
    //         for row in 1..self.height {
    //             for col in 0..self.width {
    //                 if let Some(&Tile::Round) = self.map.get(&(col, row)) {
    //                     if let Some(&Tile::Empty) = self.map.get(&(col, row - 1)) {
    //                         self.map.insert((col, row), Tile::Empty);
    //                         self.map.insert((col, row - 1), Tile::Round);
    //                         differences += 1;
    //                     }
    //                 }
    //             }
    //         }
    //         if differences == 0 {
    //             return self;
    //         }
    //     }
    // }

    // This works out to be about 12 times faster :D
    fn tip_north(mut self) -> Self {
        for row in 1..self.height {
            for col in 0..self.width {
                if let Some(&Tile::Round) = self.map.get(&(col, row)) {
                    let mut last_empty_row = None;
                    for search_row in (0..row).rev() {
                        if let Some(&Tile::Empty) = self.map.get(&(col, search_row)) {
                            last_empty_row = Some(search_row);
                        } else {
                            break;
                        }
                    }
                    if let Some(last_empty_row) = last_empty_row {
                        self.map.insert((col, row), Tile::Empty);
                        self.map.insert((col, last_empty_row), Tile::Round);
                    }
                }
            }
        }

        self
    }

    fn tip_south(mut self) -> Self {
        for row in (0..self.height - 1).rev() {
            for col in 0..self.width {
                if let Some(&Tile::Round) = self.map.get(&(col, row)) {
                    let mut last_empty_row = None;
                    for search_row in row + 1..self.height {
                        if let Some(&Tile::Empty) = self.map.get(&(col, search_row)) {
                            last_empty_row = Some(search_row);
                        } else {
                            break;
                        }
                    }
                    if let Some(last_empty_row) = last_empty_row {
                        self.map.insert((col, row), Tile::Empty);
                        self.map.insert((col, last_empty_row), Tile::Round);
                    }
                }
            }
        }

        self
    }

    fn tip_west(mut self) -> Self {
        for col in 1..self.width {
            for row in 0..self.height {
                if let Some(&Tile::Round) = self.map.get(&(col, row)) {
                    let mut last_empty_col = None;
                    for search_col in (0..col).rev() {
                        if let Some(&Tile::Empty) = self.map.get(&(search_col, row)) {
                            last_empty_col = Some(search_col);
                        } else {
                            break;
                        }
                    }
                    if let Some(last_empty_col) = last_empty_col {
                        self.map.insert((col, row), Tile::Empty);
                        self.map.insert((last_empty_col, row), Tile::Round);
                    }
                }
            }
        }

        self
    }
}

// For checking all tip directions work as intended
impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let &tile = self.map.get(&(col, row)).unwrap();
                let symbol = match tile {
                    Tile::Cube  => '#',
                    Tile::Round => 'O',
                    Tile::Empty => '.',
                };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Platform {
    Platform { 
        map: input
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.trim().chars().enumerate().map(move |(col, ch)| {
                    let tile = match ch {
                        'O' => Tile::Round,
                        '#' => Tile::Cube,
                        '.' => Tile::Empty,
                        _   => panic!("Unrecognised tile type: {}", ch),
                    };
                    ((col, row), tile)
                })
            })
            .collect(),
        width: input.lines().last().unwrap().trim().len(),
        height: input.lines().count(),
    }
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &Platform) -> usize {
    input.clone().tip_north().calculate_load()
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &Platform) -> usize {
    // println!("{}\n{}", input, input.clone().cycles(3));
    // input.clone().floyd_cycles(1_000_000_000).calculate_load()
    input.clone().brent_cycles(1_000_000_000).calculate_load()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "O....#....
                        O.OO#....#
                        .....##...
                        OO.#O....O
                        .O.....O#.
                        O.#..O.#.#
                        ..O..#O..O
                        .......O..
                        #....###..
                        #OO..#....";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 136);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(TEST)), 64);
    }
}