use std::collections::HashMap;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let mut cal: u32 = 0;
    for l in input.lines() {
        let nums: Vec<_> = l.chars()
            .filter(|n| n.is_ascii_digit())
            .map(|x| x.to_digit(10).unwrap())
            .collect();
        cal += nums.first().unwrap() * 10 + nums.last().unwrap();
    }

    cal
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let mut cal: u32 = 0;
    for l in input.lines() {
        let nums = line_parser(l);
        cal += nums.first().unwrap() * 10 + nums.last().unwrap();
    }

    cal
}

fn line_parser(line: &str) -> Vec<u32> {
    let dictionary: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three",3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let mut nums: Vec<u32> = Vec::new();
    let mut char_stack: String = String::new();

    for c in line.chars() {
        char_stack.push(c);
        if c.is_ascii_digit() {
            nums.push(c.to_digit(10).unwrap());
        } else {
            for word in dictionary.keys() {
                if char_stack.contains(word) {
                    nums.push(dictionary[word]);
                    char_stack.clear();
                    char_stack.push(c); // Oh, that was a dirty, dirty, dirty trick combining numbers like 'eightwo'
                }
            }
        }
    }
    
    nums
}

mod tests {
    use super::*;

    #[test]
    // Ensure that the dictionary lookup is working correctly
    fn test_dictionary1() {
        let text = "fplrjjznseventwocrv9";

        assert_eq!(line_parser(text), vec![7, 2, 9]);
        assert_eq!(part2(text), 79);
    }

    #[test]
    // Ensure that the dictionary lookup is working correctly
    fn test_dictionary2() {
        let text = "csdfivefhgkjfcsvsvqsrbtplhjnine7pqhpvhjqone";

        assert_eq!(line_parser(text), vec![5, 9, 7, 1]);
        assert_eq!(part2(text), 51);
    }

    #[test]
    // Test part 2
    fn test_part_deux() {
        let text = "941
            onefivejnbgncqfzcsixdqd8rxjd2
            9fivesixfivefivesix647
            noneight25fhqrvv
            eightninephmksl9dvhvcbvdldthree
            threegr8";

        assert_eq!(part2(text), 91 + 12 + 97 + 15 + 83 + 38);
    }
}