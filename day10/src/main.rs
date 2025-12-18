#[allow(dead_code)]

fn part_1(input: &str) -> u64 {
    0
}

fn part_2(input: &str) -> u64 {
    0
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let input = include_str!("example.txt");
        assert_eq!(part_1(input), 40);
    }

    #[test]
    fn test_part_one_input() {
        let input = include_str!("input.txt");
        println!("count {}", part_1(input));
    }

    // #[test]
    // fn test_part_two_example() {
    //     let input = include_str!("example.txt");
    //     assert_eq!(part_2(input), 3263827);
    // }

    // #[test]
    // fn test_part_two_input() {
    //     let input = include_str!("input.txt");
    //     println!("count {}", part_2(input));
    // }
}
