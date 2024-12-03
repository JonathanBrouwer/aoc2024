use regex::Regex;

fn part1(inp: &str) -> usize {
    let regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    regex
        .captures_iter(inp)
        .map(|m| {
            m.extract::<2>()
                .1
                .into_iter()
                .map(|v| v.parse::<usize>().unwrap())
                .product::<usize>()
        })
        .sum()
}

fn part2(inp: &str) -> usize {
    let regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|do\(\)|don't\(\)").unwrap();
    let mut enabled = true;
    regex
        .captures_iter(inp)
        .map(|m| match m.get(0).unwrap().as_str() {
            "do()" => {
                enabled = true;
                0
            }
            "don't()" => {
                enabled = false;
                0
            }
            _ => {
                if !enabled {
                    return 0;
                }
                m.iter()
                    .skip(1)
                    .map(|n| n.unwrap().as_str().parse::<usize>().unwrap())
                    .product()
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(161, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(170778545, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example2"));
        assert_eq!(48, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(82868252, result);
    }
}
