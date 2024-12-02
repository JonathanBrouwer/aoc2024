use itertools::Itertools;

fn part1(inp: &str) -> usize {
    parse_input(inp)
        .into_iter()
        .filter(|line| {
            let mut diffs = line
                .iter()
                .tuple_windows()
                .map(|(a, b)| *a as isize - *b as isize);
            diffs.clone().all(|v| (1..=3).contains(&v)) || diffs.all(|v| (-3..=-1).contains(&v))
        })
        .count()
}

fn part2(inp: &str) -> usize {
    parse_input(inp)
        .into_iter()
        .filter(|line| {
            // For any i we drop
            (0..line.len()).any(|i| {
                let line = line
                    .iter()
                    .enumerate()
                    .filter(|(j, v)| *j != i)
                    .map(|(_, v)| v);
                let mut diffs = line.tuple_windows().map(|(a, b)| *a as isize - *b as isize);
                diffs.clone().all(|v| (1..=3).contains(&v)) || diffs.all(|v| (-3..=-1).contains(&v))
            })
        })
        .count()
}

fn parse_input(inp: &str) -> Vec<Vec<usize>> {
    inp.lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(2, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(213, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"));
        assert_eq!(4, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(285, result);
    }
}
