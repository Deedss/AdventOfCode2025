// Range from , till -

#[allow(dead_code)]
use std::vec;

fn parse_ranges(input: &str) -> Vec<(u64, u64)> {
    let mut ranges = input
        .split(',')
        .map(|s| {
            let values: Vec<u64> = s
                .trim()
                .split('-')
                .map(|s| s.parse::<u64>().unwrap())
                .collect();
            (values[0], values[1])
        })
        .collect();

    ranges
}

fn part_1(input: &str) -> u64 {
    let ranges = parse_ranges(input);
    let mut result = 0u64;

    // Loop over all values
    for range in ranges.iter() {
        // Update range with steps of one (11 - 22 e.g.)
        for val in range.0..(range.1 + 1) {
            // Make it a string
            let string_val = val.to_string();
            // split in half
            let split = string_val.split_at(string_val.len() / 2);
            // compare
            if split.0.eq(split.1) {
                result += val;
            }
        }
    }

    result
}

fn part_2(input: &str) -> u64 {
    let ranges = parse_ranges(input);
    let mut result = 0u64;

    // Loop over all values
    for range in ranges.iter() {
        // Update range with steps of one (11 - 22 e.g.)
        for val in range.0..(range.1 + 1) {
            // Transform to vec of chars for chunking
            let string_val : Vec<char> = val.to_string().chars().collect();
            // get index of item till half (at least 2 occurences)
            for idx in 1..(string_val.len() / 2 + 1) {
                // build a substr/chunk
                let chunk = &string_val[0..idx];
                // Check if all chunks match eachother
                if string_val.chunks(idx).all(|s| s == chunk) {
                    result += val;
                    break;
                }
            }
        }
    }

    result
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let input = include_str!("example.txt");
        assert_eq!(part_1(input), 1227775554);
    }

    #[test]
    fn test_part_one_input() {
        let input = include_str!("input.txt");
        println!("count {}", part_1(input));
    }

    #[test]
    fn test_part_two_example() {
        let input = include_str!("example.txt");
        assert_eq!(part_2(input), 4174379265);
    }

    #[test]
    fn test_part_two_input() {
        let input = include_str!("input.txt");
        println!("count {}", part_2(input));
    }
}
