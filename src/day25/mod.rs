use itertools::Itertools;

fn part1(inp: &str) -> usize {
    let mut locks = vec![];
    let mut keys = vec![];
    for item in inp.split("\n\n") {
        let mut pattern = [0usize; 5];
        for (k, _) in item
            .lines()
            .flat_map(|line| line.chars().enumerate().filter(|(i, c)| *c == '#'))
        {
            pattern[k] += 1;
        }
        if item.starts_with("#####") {
            locks.push(pattern)
        } else {
            keys.push(pattern);
        }
    }

    locks
        .iter()
        .cartesian_product(keys.iter())
        .filter(|(lock, key)| lock.iter().zip_eq(key.iter()).all(|(l, k)| l + k <= 7))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(3, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(2586, result);
    }
}
