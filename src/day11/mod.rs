use std::collections::HashMap;

fn part1(inp: &str) -> usize {
    solve(inp, 25)
}

fn part2(inp: &str) -> usize {
    solve(inp, 75)
}

fn solve(inp: &str, count: usize) -> usize {
    let mut map: HashMap<usize, usize> = HashMap::new();
    for v in inp.split(" ").map(|n| n.parse().unwrap()) {
        *map.entry(v).or_insert(0) += 1;
    }

    for _ in 0..count {
        let mut new_map = HashMap::new();
        for (k, v) in map {
            if k == 0 {
                *new_map.entry(1).or_insert(0) += v;
                continue;
            }
            let digit_count = k.ilog10() + 1;
            if digit_count % 2 == 0 {
                let mask = 10usize.pow(digit_count / 2);
                *new_map.entry(k % mask).or_insert(0) += v;
                *new_map.entry(k / mask).or_insert(0) += v;
            } else {
                *new_map.entry(k * 2024).or_insert(0) += v;
            }
        }
        map = new_map;
    }

    map.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(55312, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(224529, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(266820198587914, result);
    }
}
