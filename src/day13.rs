// use std::collections::HashMap;

// #[derive(Debug)]
// pub struct Block {
//     block:  HashMap<(usize, usize), bool>,
//     width:  usize,
//     height: usize,
// }

// impl Block {
//     fn block_score(&self) -> usize {
//         if let Some(score) = self.find_horizontal() {
//             score * 100
//         } else if let Some(score) = self.find_vertical() {
//             score
//         } else {
//             0
//         }
//     }

//     // As horizontal is searched first, care needs to be taken for false positives
//     fn find_horizontal(&self) -> Option<usize> {
//         (0..self.height - 1).find(|&row| {
//             (0..self.height - row - 1).all(|offset| self.scan_h(row - offset, row + 1 + offset) == 0)
//         }).map(|row| row + 1)
//     }

//     fn find_vertical(&self) -> Option<usize> {
//         (0..self.width - 1).find(|&col| 
//             self.scan_v(col, col + 1) == 0
//         ).map(|col| col + 1)
//     }

//     // Purely for debugging
//     // fn print_row(&self, row: usize) -> String {
//     //     (0..self.width)
//     //         .map(|col| match self.block.get(&(col, row)).unwrap() {
//     //             true => '#',
//     //             false => '.',
//     //         })
//     //         .collect()
//     // }

//     fn scan_h(&self, top: usize, bottom: usize) -> usize {
//         (0..self.width).filter(|&col| 
//             self.block.get(&(col, top)) != self.block.get(&(col, bottom))
//         ).count()
//     }

//     fn scan_v(&self, left: usize, right: usize) -> usize {
//         (0..self.height).filter(|&row| 
//             self.block.get(&(left, row)) != self.block.get(&(right, row))
//         ).count()
//     }

//     fn transcode(&self) -> BinBlock {
//         BinBlock { 
//             rows: (0..self.height).map(|row| {
//                 let row_binary = (0..self.width)
//                     .map(|col| match self.block.get(&(col, row)).unwrap() {
//                         true  => '1',
//                         false => '0',
//                     })
//                     .collect::<String>();
//                 usize::from_str_radix(&row_binary, 2).unwrap()
//             }).collect(),
//             cols: (0..self.width).map(|col| {
//                 let col_binary = (0..self.height)
//                     .map(|row| match self.block.get(&(col, row)).unwrap() {
//                         true  => '1',
//                         false => '0',
//                     })
//                     .collect::<String>();
//                 usize::from_str_radix(&col_binary, 2).unwrap()
//             }).collect(),
//         }
//     }
// }

// Okay, I'll try this another way since this is in binary.
// Due to less 'brute forcing', this should run faster.
#[derive(Debug)]
pub struct BinBlock {
    rows: Vec<usize>,
    cols: Vec<usize>,
}

impl BinBlock {
    // fn block_score(&self, h_prev: Option<usize>, v_prev: Option<usize>) -> usize {
    //     100 * self.find_horizontal(h_prev).unwrap_or(0)
    //     + self.find_vertical(v_prev).unwrap_or(0)
    // }

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

    fn clone_with_mut(&self) -> BinBlock {
        BinBlock {
            rows: self.rows.clone(),
            cols: self.cols.clone(),
        }
    }

    // fn corrected_block_score(&self) -> usize {
    //     let h_prev = self.find_horizontal(None);
    //     let v_prev = self.find_vertical(None);

    //     self.rows.iter().enumerate().flat_map(|(row, _)| {
    //         self.cols.iter().enumerate().filter_map(move |(col, _)| {
    //             let mut test_block = self.clone_with_mut();

    //             test_block.rows[row] ^= 1 << (self.cols.len() - col - 1);
    //             test_block.cols[col] ^= 1 << (self.rows.len() - row - 1);

    //             let h_diff = test_block.find_horizontal(h_prev).unwrap_or(0);
    //             let v_diff = test_block.find_vertical(v_prev).unwrap_or(0);
    //             (h_diff != 0 || v_diff != 0).then(|| test_block.block_score(h_prev, v_prev))
    //         })
    //     }).next().unwrap_or(0)
    // }

    // Again, reflections must be tested in case there is a false positive.
    // Additionally there may be more than one reflection, which would explain
    // why my previous answer was too low.
    // fn find_horizontal(&self, prev: Option<usize>) -> Option<usize> {
    //     (0..self.rows.len() - 1).find(|&row| {
    //         let margin = (row + 1).min(self.rows.len() - row - 1);
    //         (0..margin).all(|offset| self.rows[row - offset] == self.rows[row + offset + 1])
    //     }).map(|row| row + 1).filter(|&result| prev != Some(result))
    // }

    // Add verification to verticals and find extra reflections.
    // fn find_vertical(&self, prev: Option<usize>) -> Option<usize> {
    //     (0..self.cols.len() - 1).find(|&col| {
    //         let margin = (col + 1).min(self.cols.len() - col - 1);
    //         (0..margin).all(|offset| self.cols[col - offset] == self.cols[col + offset + 1])
    //     }).map(|col| col + 1).filter(|&result| prev != Some(result))
    // }

    // Combined method?
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

// #[aoc_generator(day13)]
// pub fn input_generator(input: &str) -> Vec<BinBlock> {
//     input
//         .split("\n\n")
//         .map(|chunk| Block {
//             block: chunk
//                 .lines()
//                 .enumerate()
//                 .flat_map(|(row, line)| {
//                     line.trim()
//                         .chars()
//                         .enumerate()
//                         .map(move |(col, char)| (
//                             (col, row),
//                             match char {
//                                 '.' => false,
//                                 '#' => true,
//                                 _   => panic!("Not a recognised character."),
//                             }
//                         ))
//                 })
//                 .collect(),
//             width: chunk.lines().last().unwrap().trim().len(),
//             height: chunk.lines().count(),
//         })
//         .map(|block| block.transcode())
//         .collect()
// }

// Might as well rewrite the parser.
#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<BinBlock> {
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

            BinBlock { rows, cols }
        })
        .collect()
}


#[aoc(day13, part1)]
pub fn solve_part1(input: &Vec<BinBlock>) -> usize {
    input
        .iter()
        .map(|block| block.block_score(false))
        .sum()
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &Vec<BinBlock>) -> usize {
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