type Point = (usize, usize);

#[derive(Debug)]
pub struct SkyMap {
    pub map: Vec<Point>,
    pub empty_col: Vec<usize>,
    pub empty_row: Vec<usize>,
}

impl SkyMap {
    fn total_manhattan_distances(&self, coef: usize) -> usize {
        (0..self.map.len() - 1)
            .flat_map(|i| (i + 1..self.map.len()).map(move |j| (self.map[i], self.map[j])))
            .map(|(first, other)| self.dist_with_coef(first, other, coef))
            .sum()
    }

    fn dist_with_coef(&self, first: Point, other: Point, coef: usize) -> usize {
        let (col_min, col_max) = (first.0.min(other.0), first.0.max(other.0));
        let (row_min, row_max) = (first.1.min(other.1), first.1.max(other.1));

        let delta_col = col_max - col_min;
        let delta_row = row_max - row_min;
    
        let col_exp = self.empty_col.iter().filter(|&col| *col >= col_min && *col <= col_max).count();
        let row_exp = self.empty_row.iter().filter(|&row| *row >= row_min && *row <= row_max).count();
    
        delta_col + col_exp * (coef - 1) + delta_row + row_exp * (coef - 1)
    }
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> SkyMap {
    let mut empty_col: Vec<usize> = Vec::new();
    let mut empty_row: Vec<usize> = Vec::new();
    let mut map: Vec<Point> = Vec::new();
    let width = input.lines().last().unwrap().trim().len();

    input
        .lines()
        .enumerate()
        .for_each(|(row, line)| {
            let mut row_count = 0;
            line.trim()
                .chars()
                .enumerate()
                .for_each(|(col, char)| {
                    if char == '#' {
                        map.push((col, row));
                        row_count += 1;
                    }
                });
                if row_count == 0 {
                    empty_row.push(row);
                }
        });
    
    (0..width).for_each(|i| {
        if !map.iter().any(|&(col, _)| col == i) {
            empty_col.push(i);
        }
    });

    SkyMap { map, empty_col, empty_row }
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &SkyMap) -> usize {
    input.total_manhattan_distances(2)
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &SkyMap) -> usize {
    input.total_manhattan_distances(1_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "...#......
                        .......#..
                        #.........
                        ..........
                        ......#...
                        .#........
                        .........#
                        ..........
                        .......#..
                        #...#.....";

    #[test]
    fn generator_test() {
        assert_eq!(input_generator(TEST).map, vec![(3, 0), (7, 1), (0, 2), (6, 4), (1, 5), (9, 6), (7, 8), (0, 9), (4, 9)]);
        assert_eq!(input_generator(TEST).empty_col, vec![2, 5, 8]);
        assert_eq!(input_generator(TEST).empty_row, vec![3, 7]);
    }

    #[test]
    fn part1_test() {
        assert_eq!(solve_part1(&input_generator(TEST)), 374);
    }
}