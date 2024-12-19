use itertools::Itertools;
use regex::Regex;

fn part1(inp: &str) -> usize {
    let (towels, designs) = parse_input(inp);

    let regex = Regex::new(&format!("^({})*$", towels.join("|"))).unwrap();
    designs
        .into_iter()
        .filter(|design| regex.is_match(design))
        .count()
}

fn part2(inp: &str) -> usize {
    let (towels, designs) = parse_input(inp);

    designs
        .into_iter()
        .map(|design| {
            let mut possibilities = vec![0; design.len() + 1];
            possibilities[0] = 1;

            for i in 1..=design.len() {
                for &towel in &towels {
                    if towel.len() > i {
                        continue;
                    }
                    if &design[i - towel.len()..i] != towel {
                        continue;
                    }
                    possibilities[i] += possibilities[i - towel.len()];
                }
            }

            possibilities[design.len()]
        })
        .sum()
}

fn parse_input(inp: &str) -> (Vec<&str>, Vec<&str>) {
    let (p1, p2) = inp.split_once("\n\n").unwrap();
    (p1.split(", ").collect_vec(), p2.lines().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(6, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(258, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"));
        assert_eq!(16, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(632423618484345, result);
    }
}
