#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line
                .trim()
                .split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

// Hmm... I don't like this redundancy, but I also don't like recursive calls...
fn extrapolate(series: &Vec<i32>) -> i32 {
    let mut all_zeroes = true;
    let mut deltas: Vec<i32> = series.windows(2)
        .map(|window| {
            let delta = window[1] - window[0];
            all_zeroes = all_zeroes && delta == 0;
            delta
        })
        .collect();

    let mut end_num = *series.last().unwrap();

    while !all_zeroes {
        let current_deltas = deltas.windows(2)
            .map(|window| {
                let delta = window[1] - window[0];
                all_zeroes = all_zeroes && delta == 0;
                delta
            })
            .collect();

        if deltas.is_empty() {
            break;
        }

        end_num += deltas.last().unwrap_or(&0);

        deltas = current_deltas;
    }

    end_num
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &Vec<Vec<i32>>) -> i32 {
    input.iter().map(|series| extrapolate(&series)).sum()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &Vec<Vec<i32>>) -> i32 {
    input.iter().map(|series| extrapolate(&series.iter().rev().cloned().collect())).sum()
}