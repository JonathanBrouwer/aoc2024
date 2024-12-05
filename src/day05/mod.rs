use std::cmp::Ordering;
use std::collections::HashSet;
use itertools::Itertools;
use petgraph::prelude::*;

fn part1(inp: &str) -> usize {
    let (constraints, tests) = parse_input(inp);

    let mut sum = 0;
    'test: for test in tests {
        for i in 0..test.len() {
            for j in i+1 .. test.len() {
                if constraints.contains(&(test[j], test[i])) {
                    continue 'test;
                }
            }
        }
        sum += test[test.len() / 2];
    }

    sum
}

fn part2(inp: &str) -> usize {
    let (constraints, tests) = parse_input(inp);
    let mut sum = 0;
    for mut test in tests {
        let mut correct = true;
        for i in 0..test.len() {
            for j in i+1 .. test.len() {
                if constraints.contains(&(test[j], test[i])) {
                    correct = false;
                }
            }
        }
        if correct {
            continue
        }

        test.sort_by(|a, b| {
            if constraints.contains(&(*a, *b)) {
                Ordering::Less
            } else if constraints.contains(&(*b, *a)) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
        sum += test[test.len()/2];
    }

    sum
}

fn parse_input(inp: &str) -> (HashSet<(usize, usize)>, Vec<Vec<usize>>) {
    let (pairs, rules) = inp.split_once("\n\n").unwrap();
    let pairs = pairs.lines().map(|line| {
        let (a, b) = line.split_once("|").unwrap();
        (a.parse().unwrap(), b.parse().unwrap())
    }).collect();

    let tests = rules.lines().map(|line| line.split(",").map(|v| v.parse().unwrap()).collect()).collect();

    (pairs, tests)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(143, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(5964, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"));
        assert_eq!(123, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(4719, result);
    }
}
