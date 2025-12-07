use std::str::Chars;

#[allow(dead_code)]

fn get_starting_point(vec: Vec<char>) -> (usize, usize) {
    (
        0,
        vec.iter().enumerate().find(|&(_, c)| *c == 'S').unwrap().0,
    )
}

fn transform(grid: &mut Vec<Vec<char>>) -> u64 {
    let mut sum = 0;
    for row in 0..grid.len() - 1 {
        if row + 1 > grid.len() - 1 {
            break;
        }

        for col in 0..grid[row].len() {
            let c = grid[row][col];

            if c == 'S' {
                grid[row + 1][col] = '|';
            } else if c == '|' {
                if grid[row + 1][col] == '^' {
                    if col > 0 && col < grid[row].len() {
                        sum += 1; // split
                        grid[row + 1][col - 1] = '|';
                        grid[row + 1][col + 1] = '|';
                    }
                } else {
                    grid[row + 1][col] = '|';
                }
            }
        }
    }
    sum
}

fn part_1(input: &str) -> u64 {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    transform(&mut grid)
}

fn part_2(input: &str) -> u64 {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    transform(&mut grid);

    // Use dfs to search all paths
    // not yet sure how

    0
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let input = include_str!("example.txt");
        assert_eq!(part_1(input), 21);
    }

    #[test]
    fn test_part_one_input() {
        let input = include_str!("input.txt");
        println!("count {}", part_1(input));
    }

    #[test]
    fn test_part_two_example() {
        let input = include_str!("example.txt");
        assert_eq!(part_2(input), 40);
    }

    #[test]
    fn test_part_two_input() {
        let input = include_str!("input.txt");
        println!("count {}", part_2(input));
    }
}
