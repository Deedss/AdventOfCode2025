#[allow(dead_code)]

fn get_starting_point(vec: Vec<char>) -> (usize, usize) {
    (
        0,
        vec.iter().enumerate().find(|&(_, c)| *c == 'S').unwrap().0,
    )
}
    
fn transform(input: &Vec<Vec<char>>) -> (Vec<Vec<char>>, u64) {
    let rows = input.len();
    let cols = input[0].len();

    // clean pipe grid
    let mut pipe = vec![vec!['.'; cols]; rows];

    // find start S
    let (sr, sc) = get_starting_point(input[0].clone());
    pipe[sr][sc] = "S".chars().next().unwrap();

    let mut splits = 0;

    for r in 0..rows - 1 {
        for c in 0..cols {
            let cur = pipe[r][c];

            match cur {
                'S' => {
                    pipe[r + 1][c] = '|';
                }
                '|' => {
                    if input[r + 1][c] == '^' {
                        splits += 1;
                        if c > 0 {
                            pipe[r + 1][c - 1] = '|';
                        }
                        if c + 1 < cols {
                            pipe[r + 1][c + 1] = '|';
                        }
                    } else {
                        pipe[r + 1][c] = '|';
                    }
                }
                _ => {}
            }
        }
    }

    (pipe, splits)
}



fn part_1(input: &str) -> u64 {
    let input_grid: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();

    let (_pipe, splits) = transform(&input_grid);

    splits
}

fn part_2(input: &str) -> u64 {
    let input_grid: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();

    let (pipe, _splits) = transform(&input_grid);

    let rows = pipe.len();
    let cols = pipe[0].len();

    let mut dp = vec![vec![0u64; cols]; rows];

    // locate S
    let (sr, sc) = get_starting_point(input_grid[0].clone());
    dp[sr][sc] = 1;

    for r in 0..rows - 1 {
        for c in 0..cols {
            let ways = dp[r][c];
            if ways == 0 { continue; }

            match pipe[r][c] {
                'S' | '|' => {
                    // splitter?
                    if input_grid[r + 1][c] == '^' {
                        if c > 0 {
                            dp[r + 1][c - 1] += ways;
                        }
                        if c + 1 < cols {
                            dp[r + 1][c + 1] += ways;
                        }
                    } else {
                        dp[r + 1][c] += ways;
                    }
                }
                _ => {}
            }
        }
    }

    // sum all bottom-row ways
    dp[rows - 1].iter().sum()
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
