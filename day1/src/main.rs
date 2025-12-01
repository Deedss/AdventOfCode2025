fn answer_1(input: &str) -> u64 {
    let mut count = 0;

    let mut dial: i32 = 50;
    for line in input.lines() {
        let (dir, val_str) = line.split_at(1);
        let dist: i32 = val_str.parse::<i32>().unwrap();

        match dir {
            "L" => dial -= dist,
            "R" => dial += dist,
            _ => unreachable!(),
        }

        // wrap around 0–99
        dial = (dial % 100 + 100) % 100;

        if dial == 0 {
            count += 1;
        }
    }

    count
}

fn answer_2(input: &str) -> u64 {
    let mut count = 0;

    let mut dial: i32 = 50;
    for line in input.lines() {
        let (dir, val_str) = line.split_at(1);
        let dist = val_str.parse::<i32>().unwrap();

        let step = match dir {
            "L" => -1,
            "R" => 1,
            _ => unreachable!(),
        };

        for _ in 0..dist {
            dial = (dial + step + 100) % 100;
            if dial == 0 {
                count += 1;
            }
        }
    }

    count
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let input = include_str!("example.txt");
        assert_eq!(answer_1(input), 3);
    }

    #[test]
    fn part_1() {
        let input = include_str!("input.txt");
        println!("count {}", answer_1(input));
        assert_eq!(answer_1(input), 1141);
    }

    #[test]
    fn part_2_example() {
        let input = include_str!("example.txt");
        assert_eq!(answer_2(input), 6);
    }

    #[test]
    fn part_2() {
        let input = include_str!("input.txt");
        println!("Sum is {}", answer_2(input));
    }
}
