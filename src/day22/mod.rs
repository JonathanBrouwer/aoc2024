use std::collections::{HashMap, HashSet};

fn part1(inp: &str) -> usize {
    parse_input(inp).into_iter().map(|mut num| {
        for _ in 0..2000 {
            num = next(num);
        }
        num
    }).sum()
}

fn part2(inp: &str) -> usize {
    let mut scores = HashMap::new();

    for n0 in parse_input(inp) {
        let n1 = next(n0);
        let n2 = next(n1);
        let n3 = next(n2);
        let mut num = n3;

        let n0 = (n0 % 10) as isize;
        let n1 = (n1 % 10) as isize;
        let n2 = (n2 % 10) as isize;
        let n3 = (n3 % 10) as isize;

        let mut window = [
            0,
            n1 - n0,
            n2 - n1,
            n3 - n2,
        ];
        let mut seen = HashSet::new();

        for _ in 0..2000 - 4 {
            window.rotate_left(1);
            let next_num = next(num);
            window[3] = (next_num % 10) as isize - (num % 10) as isize;
            num = next_num;

            if !seen.insert(window) {
                continue;
            }

            *scores.entry(window).or_insert(0) += num % 10;
        }
    }

    *scores.values().max().unwrap()
}

fn next(mut num: usize) -> usize {
    num ^= num << 6;
    num &= 0xFFFFFF;
    num ^= num >> 5;
    num ^= num << 11;
    num &= 0xFFFFFF;
    num
}

fn parse_input(inp: &str) -> Vec<usize> {
    inp.lines().map(|line| line.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(37327623, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(14691757043, result);
    }

    #[test]
    fn test_part2_ex2() {
        let result = part2(include_str!("example2"));
        assert_eq!(23, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(1831, result);
    }
}
