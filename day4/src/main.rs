#[allow(dead_code)]

// [][][][][]
// [][][][][]
// [][][][][]
// [][][][][]
// [][][][][]
fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn is_valid(arr: &Vec<Vec<char>>, row: isize, col: isize) -> bool {
    // Check all corners
    let mut rolls_in_range = 0;

    // 
    for r in -1..2 {
        for c in -1..2 {
            // If itself continue 
            if r == 0 && c == 0 {
                continue;
            }

            // row or col can be done
            if (row + r < 0) || (col + c < 0) {
                continue;
            }

            let (c_row, c_col) = ((row + r) as usize, (col + c) as usize);
            // Only check in bounds
            if arr.get(c_row).is_some_and(|inner| inner.get(c_col).is_some_and(|c| *c == '@')) {
                rolls_in_range += 1;
            } 
        }
    }

    rolls_in_range < 4
}

fn part_1(input: &str) -> u64 {
    let arr = parse_input(input);
    let mut count = 0;

    for (row, v_row) in arr.iter().enumerate() {
        for (col, &v_col) in v_row.iter().enumerate() {
            if v_col == '@' {
                if is_valid(&arr, row.try_into().unwrap(), col.try_into().unwrap()) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn part_2(input: &str) -> u64 {
    let arr = parse_input(input);
    let mut sum = 0;

    sum
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let input = include_str!("example.txt");
        assert_eq!(part_1(input), 13);
    }

    #[test]
    fn test_part_one_input() {
        let input = include_str!("input.txt");
        println!("count {}", part_1(input));
    }

    // #[test]
    // fn test_part_two_example() {
    //     let input = include_str!("example.txt");
    //     assert_eq!(part_2(input), 3121910778619);
    // }

    // #[test]
    // fn test_part_two_input() {
    //     let input = include_str!("input.txt");
    //     println!("count {}", part_2(input));
    // }
}
