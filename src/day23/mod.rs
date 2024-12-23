use itertools::Itertools;
use std::collections::HashMap;

fn part1(inp: &str) -> usize {
    let edges = parse_input(inp);
    edges
        .iter()
        .map(|(c1, cn)| {
            cn.iter()
                .tuple_combinations()
                .filter(|(c2, c3)| edges[*c2].contains(c3))
                .filter(|(c2, c3)| {
                    c1.starts_with('t') || c2.starts_with('t') || c3.starts_with('t')
                })
                .count()
        })
        .sum::<usize>()
        / 3
}

fn part2(inp: &str) -> String {
    let edges = parse_input(inp);

    let mut best = vec![];
    for (n1, ns) in &edges {
        for i in 0usize..(1 << ns.len()) {
            let mut nodes = vec![];
            for j in 0..ns.len() {
                if i & (1 << j) == 0 {
                    continue;
                }
                nodes.push(ns[j]);
            }
            if nodes.len() + 1 <= best.len() {
                continue;
            }
            if !nodes
                .iter()
                .tuple_combinations()
                .all(|(c2, c3)| edges[*c2].contains(c3))
            {
                continue;
            }
            nodes.push(n1);
            best = nodes;
        }
    }
    best.sort();
    best.join(",")
}

fn parse_input(inp: &str) -> HashMap<&str, Vec<&str>> {
    inp.lines()
        .flat_map(|line| {
            let (a, b) = line.split_once("-").unwrap();
            [(a, b), (b, a)]
        })
        .into_group_map()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(7, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(1368, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"));
        assert_eq!("co,de,ka,ta", result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!("dd,ig,il,im,kb,kr,pe,ti,tv,vr,we,xu,zi", result);
    }
}
