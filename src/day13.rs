#[derive(Debug)]
pub struct Block {
    rows: Vec<usize>,
    cols: Vec<usize>,
}

impl Block {
    fn block_score(&self, smudged: bool) -> usize {
        match smudged {
            false => {
                match self.find_mirror((None,None)).unwrap() {
                    (Some(score), None) => 100 * score,
                    (None, Some(score)) => score,
                    _ => unreachable!(),
                }
            },
            true  => {
                match self.find_clean().unwrap() {
                    (Some(score), None) => 100 * score,
                    (None, Some(score)) => score,
                    _ => unreachable!(),
                }
            },
        }
    }

    fn clone_with_mut(&self) -> Block {
        Block {
            rows: self.rows.clone(),
            cols: self.cols.clone(),
        }
    }

    fn find_mirror(&self, prev: (Option<usize>, Option<usize>)) -> Option<(Option<usize>, Option<usize>)> {
        let mut row = 0;
        while row < self.rows.len() - 1 {
            let margin = (row + 1).min(self.rows.len() - row - 1);

            let reflected = (0..margin).all(|offset| self.rows[row - offset] == self.rows[row + offset + 1]);

            if reflected {
                let result = (Some(row + 1), None);
                if result != prev {
                    return Some(result);
                }
            }
            row += 1;
        }

        let mut col = 0;
        while col < self.cols.len() - 1 {
            let margin = (col + 1).min(self.cols.len() - col - 1);

            let reflected = (0..margin).all(|offset| self.cols[col - offset] == self.cols[col + offset + 1]);

            if reflected {
                let result = (None, Some(col + 1));
                if result != prev {
                    return Some(result);
                }
            }
            col += 1;
        }

        None
    }

    fn find_clean(&self) -> Option<(Option<usize>, Option<usize>)> {
        let prev = self.find_mirror((None, None)).unwrap();

        for row in 0..self.rows.len() {
            for col in 0..self.cols.len() {
                let mut test_block = self.clone_with_mut();

                test_block.rows[row] ^= 1 << (self.cols.len() - col - 1);
                test_block.cols[col] ^= 1 << (self.rows.len() - row - 1);

                match test_block.find_mirror(prev) {
                    None => continue,
                    Some(result) => return Some(result),
                }
            }
        }

        None
    }
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<Block> {
    input
        .split("\n\n")
        .map(|chunk| {
            // Why parse shit twice when I can do both simultaneously?
            let width = chunk.lines().last().unwrap().trim().len();
            let (rows, cols) = chunk
                .lines()
                .fold((Vec::new(), vec![String::new(); width]),
            |(mut rows, mut col_vec), line| {
                    let mut row_binary = String::new();
                    for (col, char) in line.trim().chars().enumerate() {
                        let ch = match char {
                            '.' => '0',
                            '#' => '1',
                            _   => panic!("Invalid character."),
                        };
                        row_binary.push(ch);
                        col_vec[col].push(ch);
                    }
                    rows.push(usize::from_str_radix(&row_binary, 2).unwrap());
                    (rows, col_vec)
                });
            let cols = cols.into_iter()
                .map(|column| usize::from_str_radix(&column, 2).unwrap())
                .collect();

            Block { rows, cols }
        })
        .collect()
}


#[aoc(day13, part1)]
pub fn solve_part1(input: &Vec<Block>) -> usize {
    input
        .iter()
        .map(|block| block.block_score(false))
        .sum()
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &Vec<Block>) -> usize {
    input
        .iter()
        .map(|block| block.block_score(true))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "#.##..##.
                        ..#.##.#.
                        ##......#
                        ##......#
                        ..#.##.#.
                        ..##..##.
                        #.#.##.#.

                        #...##..#
                        #....#..#
                        ..##..###
                        #####.##.
                        #####.##.
                        ..##..###
                        #....#..#";

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 405);
    }

    #[test]
    fn part2_test() {
        assert_eq!(solve_part2(&input_generator(TEST)), 400);
    }
}