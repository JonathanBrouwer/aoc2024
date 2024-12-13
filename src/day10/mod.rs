use std::collections::{HashSet, VecDeque};

fn part1(inp: &str) -> usize {
    solve(inp, false)
}

fn part2(inp: &str) -> usize {
    solve(inp, true)
}

fn solve(inp: &str, part2: bool) -> usize {
    let graph: Vec<Vec<u32>> = inp
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut solutions = 0;
    for y in 0..graph.len() {
        for x in 0..graph[y].len() {
            if graph[y][x] == 0 {
                let mut bfs = VecDeque::new();
                bfs.push_back((y, x));

                let mut seen = HashSet::new();
                while let Some((y, x)) = bfs.pop_back() {
                    if graph[y][x] == 9 {
                        solutions += 1;
                        continue;
                    }
                    for (ny, nx) in [
                        (y + 1, x),
                        (y.wrapping_sub(1), x),
                        (y, x + 1),
                        (y, x.wrapping_sub(1)),
                    ] {
                        if let Some(v) = graph.get(ny).and_then(|row| row.get(nx)) {
                            if *v == graph[y][x] + 1 {
                                if !seen.insert((ny, nx)) && !part2 {
                                    continue;
                                }
                                bfs.push_back((ny, nx));
                            }
                        }
                    }
                }
            }
        }
    }

    solutions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(1, result);
    }

    #[test]
    fn test_part1_ex2() {
        let result = part1(include_str!("example2"));
        assert_eq!(36, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(644, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example2"));
        assert_eq!(81, result);
    }

    #[test]
    fn test_part2_ex4() {
        let result = part2(include_str!("example4"));
        assert_eq!(3, result);
    }

    #[test]
    fn test_part2_ex3() {
        let result = part2(include_str!("example3"));
        assert_eq!(227, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(1366, result);
    }
}
