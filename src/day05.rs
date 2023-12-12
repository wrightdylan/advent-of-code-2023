use rayon::prelude::*;

// Vec<usize> is just a list of all seed numbers
// Vec<Vec<Map>> is a vector of all maps in the order of seed2dirt, dirt2shit, shit2aqua, aqua2lux, lux2temp, temp2damp, damp2loc
// Would Vec<Vec<Vec<usize>>> be taking the piss just a little?
// On second thought, let's take the piss.
#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> (Vec<usize>, Vec<Vec<Vec<usize>>>) {
    input
        .split_once("\n\n")
        .map(|(first, second)|
            (first
                .split(':')
                .last()
                .unwrap()
                .split_whitespace()
                .map(|seed| seed.parse().unwrap())
                .collect(),
            second
                .split("\n\n")
                .map(|maps| {
                    maps.lines()
                        .skip(1)
                        .map(|map| {
                            map
                                .split_whitespace()
                                .map(|num| num.parse().unwrap())
                                .collect()
                        })
                        .collect()
                })
                .collect()))
        .unwrap()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &(Vec<usize>, Vec<Vec<Vec<usize>>>)) -> usize {
    let seeds = &input.0;
    let cats = &input.1;

    let locs: Vec<usize> = seeds
        .iter()
        .map(|&seed| {
            let mut result = seed;
            for maps in cats {
                for map in maps {
                    if result >= map[1] && result < map[1] + map[2] {
                        result = map[0] + result - map[1];
                        break; // Second test case revealed a dirty, dirty trap
                    }
                }
            }
            result
        })
        .collect();

    *locs.iter().min().unwrap()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &(Vec<usize>, Vec<Vec<Vec<usize>>>)) -> usize {
    // Run out of memory doing it this way.
    // Maybe I'll try it on my desktop, but I'll split it for my laptop.
    // let seeds: Vec<usize> = input.0
    //     .as_slice()
    //     .chunks_exact(2)
    //     .flat_map(|ch| (ch[0]..ch[0]+ch[1]))
    //     .collect();
    let seeds = &input.0;
    let cats = &input.1;

    seeds
        .chunks_exact(2)
        .map(|ch| {
            let batch = (ch[0]..ch[0] + ch[1]).collect::<Vec<usize>>();
            let locs: Vec<usize> = batch
                .par_iter()
                .map(|&seed| {
                    let mut result = seed;
                    for maps in cats {
                        for map in maps {
                            if result >= map[1] && result < map[1] + map[2] {
                                result = map[0] + result - map[1];
                                break;
                            }
                        }
                    }
                    result
                })
                .collect();

            *locs.iter().min().unwrap()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48

    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15

    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4

    water-to-light map:
    88 18 7
    18 25 70

    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13

    temperature-to-humidity map:
    0 69 1
    1 0 69

    humidity-to-location map:
    60 56 37
    56 93 4";

    #[test]
    fn part1() {
        assert_eq!(solve_part1(&input_generator(TEST)), 35);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(&input_generator(TEST)), 46);
    }
}