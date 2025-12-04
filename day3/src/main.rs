#[allow(dead_code)]

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    let mut banks: Vec<Vec<u64>> = vec![];

    for line in input.lines() {
        banks.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect(),
        );
    }

    banks
}

fn get_largest_first_val_and_idx(arr: &[u64]) -> (usize, u64) {
    let (revidx, &first_val) = arr
        .iter()
        .rev()
        .enumerate()
        .max_by_key(|&(_, v)| v)
        .unwrap();

    let idx = arr.len() - (revidx); // get actual idx 
    (idx, first_val)
}

fn part_1(input: &str) -> u64 {
    let banks = parse_input(input);
    let mut sum = 0;

    // Loop over banks
    for bank in banks {
        // Find highest value between 0..len() - 1
        let (idx, first_val) = get_largest_first_val_and_idx(&bank[..&bank.len() - 1]);
        let last_val = bank[idx..].iter().max().unwrap();

        sum += (first_val * 10) + last_val;
    }

    sum
}

fn part_2(input: &str) -> u64 {
    let banks = parse_input(input);
    let mut sum = 0;

    for bank in banks {
        let mut tmp = String::new();
        let mut global_idx;

        let search_len = bank.len() - 12;
        let (local_idx, val) = get_largest_first_val_and_idx(&bank[0..search_len]);

        global_idx = local_idx;
        tmp.push_str(&val.to_string());

        while tmp.len() < 12 {
            let slice = &bank[global_idx..bank.len() - (11 - tmp.len())];
            let (local_idx, new_val) = get_largest_first_val_and_idx(slice);

            global_idx += local_idx;
            tmp.push_str(&new_val.to_string());
        }
        sum += tmp.parse::<u64>().unwrap();
    }

    sum
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let input = include_str!("example.txt");
        assert_eq!(part_1(input), 357);
    }

    #[test]
    fn test_part_one_input() {
        let input = include_str!("input.txt");
        println!("count {}", part_1(input));
        assert_eq!(part_1(input), 17330);
    }

    #[test]
    fn test_part_two_example() {
        let input = include_str!("example.txt");
        assert_eq!(part_2(input), 3121910778619);
    }

    #[test]
    fn test_part_two_input() {
        let input = include_str!("input.txt");
        println!("Sum is {}", part_2(input));
        assert_eq!(part_2(input), 171518260283767);
    }
}
