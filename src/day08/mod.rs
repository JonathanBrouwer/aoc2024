use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn part1(inp: &str) -> usize {
    let (size_y, size_x, coords) = parse_input(inp);

    let mut set = HashSet::<(usize, usize)>::new();
    for (_, points) in coords {
        for (&(y1, x1), &(y2, x2)) in points.iter()
            .cartesian_product(points.iter())
            .filter(|(l,r)| l!=r) {
            let y3 = (2 * y1).wrapping_sub(y2);
            let x3 = (2 * x1).wrapping_sub(x2);
            if y3 < size_y && x3 < size_x {
                set.insert((y3, x3));
            }
        }
    }

    set.len()
}

fn part2(inp: &str) -> usize {
    let (size_y, size_x, coords) = parse_input(inp);

    let mut set = HashSet::<(usize, usize)>::new();
    for (_, points) in coords {
        for (&(y1, x1), &(y2, x2)) in points.iter()
            .cartesian_product(points.iter())
            .filter(|(l,r)| l!=r) {
            for i in 0.. {
                let y3 = ((i + 1) * y1).wrapping_sub(i * y2);
                let x3 = ((i + 1) * x1).wrapping_sub(i * x2);
                if y3 < size_y && x3 < size_x {
                    set.insert((y3, x3));
                } else {
                    break;
                }
            }
        }
    }

    set.len()
}

fn parse_input(inp: &str) -> (usize, usize, HashMap<char, Vec<(usize, usize)>>) {
    let size_y = inp.lines().count();
    let size_x = inp.lines().next().unwrap().len();
    let coords = inp
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '.' => None,
                c => Some((c, (y, x))),
            })
        })
        .into_group_map();
    (size_y, size_x, coords)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(14, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(390, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"));
        assert_eq!(34, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(1246, result);
    }
}
