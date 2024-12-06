use itertools::Itertools;

fn part1(inp: &str) -> usize {
    let (mut v1, mut v2) = parse_input(inp);
    v1.sort();
    v2.sort();
    v1.into_iter()
        .zip(v2.into_iter())
        .map(|(v1, v2)| v1.abs_diff(v2))
        .sum()
}

fn part2(inp: &str) -> usize {
    let (v1, v2) = parse_input(inp);
    let v2 = v2.into_iter().counts();
    v1.into_iter().map(|v| v * v2.get(&v).unwrap_or(&0)).sum()
}

fn parse_input(inp: &str) -> (Vec<usize>, Vec<usize>) {
    inp.lines()
        .map(|l| {
            let (v1, v2) = l.split_once("   ").unwrap();
            (v1.parse::<usize>().unwrap(), v2.parse::<usize>().unwrap())
        })
        .unzip()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(11, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(3246517, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"));
        assert_eq!(31, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(29379307, result);
    }
}
