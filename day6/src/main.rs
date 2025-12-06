use std::{str::Chars, u64, vec};

#[allow(dead_code)]

struct Calc {
    values: Vec<u64>,
    operator: char,
}

fn get_values(input: &str) -> Vec<Vec<u64>> {
    // Parse all lines except the last one
    let tmp: Vec<Vec<u64>> = input
        .lines()
        .take(input.lines().count() - 1)
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect()
        })
        .collect();

    // Transpose tmp: [row][col] -> [col][row]
    let mut transposed = vec![vec![0; tmp.len()]; tmp[0].len()];
    for (i, row) in tmp.iter().enumerate() {
        for (j, &val) in row.iter().enumerate() {
            transposed[j][i] = val;
        }
    }

    transposed
}

fn get_operators(input: &str) -> Vec<&str> {
    if let Some(operators) = input.lines().last() {
        operators.split_whitespace().collect()
    } else {
        vec![]
    }
}

fn get_multiplied(vec: Vec<u64>) -> u64 {
    let mut val = 1;
    for i in vec.iter() {
        val *= i;
    }
    val
}

fn part_1(input: &str) -> u64 {
    let values = get_values(input);
    let operators = get_operators(input);
    let mut sum = 0;

    for (idx, &op) in operators.iter().enumerate() {
        if let Some(vals) = values.get(idx) {
            match op {
                "+" => sum += vals.iter().sum::<u64>(),
                "*" => sum += get_multiplied(vals.to_vec()),
                _ => panic!(),
            }
        }
    }

    sum
}

fn get_values_part2(input: &str) -> Vec<Vec<u64>> {
    vec![]
}

fn part_2(input: &str) -> u64 {
    let values = get_values_part2(input);
    let operators = get_operators(input);

    let mut sum = 0;

    for (idx, &op) in operators.iter().enumerate() {
        if let Some(vals) = values.get(idx) {
            match op {
                "+" => sum += vals.iter().sum::<u64>(),
                "*" => sum += get_multiplied(vals.to_vec()),
                _ => panic!("Unknown operator"),
            }
        }
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
        assert_eq!(part_1(input), 4277556);
    }

    #[test]
    fn test_part_one_input() {
        let input = include_str!("input.txt");
        println!("count {}", part_1(input));
    }

    #[test]
    fn test_part_two_example() {
        let input = include_str!("example.txt");
        assert_eq!(part_2(input), 3263827);
    }

    // #[test]
    // fn test_part_two_input() {
    //     let input = include_str!("input.txt");
    //     println!("count {}", part_2(input));
    // }
}
