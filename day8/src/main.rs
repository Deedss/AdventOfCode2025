use std::vec;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Junction {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Circuit {
    junctions: Vec<Junction>,
}

impl Circuit {
    pub fn has_junction(&self, j: Junction) -> bool {
        self.junctions.iter().find(|&jun| *jun == j).is_some()
    }
}

fn parse_input(input: &str) -> Vec<Junction> {
    input
        .lines()
        .map(|line| {
            let vals: Vec<f64> = line
                .split(',')
                .map(|val| val.parse::<f64>().unwrap())
                .collect();
            Junction {
                x: vals[0],
                y: vals[1],
                z: vals[2],
            }
        })
        .collect()
}

/**
 * Euclidean distance
 * d(p,q) = sqrt( (p.x - q.x)^2 + (p.y - q.y)^2 + (p.z - q.z)^2 )
 */
fn distance(p: Junction, q: Junction) -> u64 {
    let dx = (p.x - q.x) * (p.x - q.x);
    let dy = (p.y - q.y) * (p.y - q.y);
    let dz = (p.z - q.z) * (p.z - q.z);

    f64::sqrt(dx + dy + dz).ceil() as u64
}

fn part_1(input: &str) -> u64 {
    let junctions = parse_input(input);
    let mut distances: Vec<(Junction, Junction, u64)> = vec![];
    let mut circuits: Vec<Circuit> = vec![];

    for i in 0..junctions.len() - 1 {
        let cur = junctions[i];
        for j in i + 1..junctions.len() {
            distances.push((cur, junctions[j], distance(cur, junctions[j])));
        }
    }

    // sort by connection size
    distances.sort_by(|a, b| a.2.cmp(&b.2));

    let mut remaining: Vec<_> = distances.iter().collect();
    while let Some(cur) = remaining.pop() {
        let (a, b, _) = *cur;

        if circuits
            .iter()
            .any(|c| c.has_junction(a) && c.has_junction(b))
        {
            continue;
        }

        if circuits.len() == 0 {
            circuits.push(Circuit {
                junctions: vec![a, b],
            });
            continue;
        }

        let mut appended = false;
        for i in 0..circuits.len() {
            let has_a = circuits[i].has_junction(a);
            let has_b = circuits[i].has_junction(b);

            match (has_a, has_b) {
                (true, true) => {
                    continue;
                }
                (true, false) => {
                    circuits[i].junctions.push(b);
                    appended = true;
                    break;
                }
                (false, true) => {
                    circuits[i].junctions.push(a);
                    appended = true;
                    break;
                }
                (false, false) => {
                    break;
                }
            }
        }
        if !appended {
            circuits.push(Circuit {
                junctions: vec![a, b],
            });
        }
    }

    for circuit in circuits.iter() {
        println!("{:?}", circuit.junctions);
    }

    // calculate multiplication
    0
}

fn part_2(input: &str) -> u64 {
    let junctions = parse_input(input);

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
