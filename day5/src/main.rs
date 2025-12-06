#[allow(dead_code)]
#[derive(Debug)]
struct Data {
    ranges: Vec<(u64, u64)>,
    ids: Vec<u64>,
}

fn parse_input(input: &str) -> Data {
    let mut data: Data = Data {
        ranges: vec![],
        ids: vec![],
    };

    for line in input.lines() {
        if line.find('-').is_some() {
            let split: Vec<u64> = line.split('-').map(|c| c.parse::<u64>().unwrap()).collect();
            data.ranges.push((split[0], split[1]));
        } else {
            if !line.is_empty() {
                data.ids.push(line.chars().as_str().parse::<u64>().unwrap());
            }
        }
    }

    data
}

fn part_1(input: &str) -> u64 {
    let data = parse_input(input);
    let mut sum = 0;

    for id in data.ids.iter() {
        for range in data.ranges.iter() {
            if range.0 <= *id && *id <= range.1 {
                sum += 1;
                break;
            }
        }
    }

    sum
}

fn part_2(input: &str) -> u64 {
    let mut data = parse_input(input);
    let mut merged: Vec<(u64, u64)> = Vec::new();

    data.ranges.sort_by_key(|r| r.0);

    for (start, end) in data.ranges {
        if let Some(last) = merged.last_mut() {
            if start <= last.1 + 1 {
                last.1 = last.1.max(end);
                continue;
            }
        }
        merged.push((start, end));
    }

    merged
        .iter()
        .map(|(s, e)| e - s + 1)
        .sum()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let input = include_str!("example.txt");
        assert_eq!(part_1(input), 3);
    }

    #[test]
    fn test_part_one_input() {
        let input = include_str!("input.txt");
        println!("count {}", part_1(input));
    }

    #[test]
    fn test_part_two_example() {
        let input = include_str!("example.txt");
        assert_eq!(part_2(input), 14);
    }

    #[test]
    fn test_part_two_input() {
        let input = include_str!("input.txt");
        println!("count {}", part_2(input));
    }
}
