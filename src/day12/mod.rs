use itertools::Itertools;
use std::collections::HashSet;
use std::ops::Index;

fn part1(inp: &str) -> usize {
    let mut visited = HashSet::<GridCoord>::new();
    let map = parse_input(inp);

    let mut total = 0;
    for start in GridCoord::coords(&map) {
        if !visited.insert(start) {
            continue;
        }
        let mut area = 0;
        let mut perimiter = 0;

        let mut bfs = vec![start];
        while let Some(next) = bfs.pop() {
            area += 1;
            perimiter += 4;
            for nb in next.neighbours_in(&map) {
                if map[nb] != map[next] {
                    continue;
                }
                perimiter -= 1;
                if !visited.insert(nb) {
                    continue;
                }
                bfs.push(nb);
            }
        }
        total += area * perimiter;
    }

    total
}

fn part2(inp: &str) -> usize {
    let mut visited = HashSet::<GridCoord>::new();
    let map = parse_input(inp);

    let mut total = 0;
    for start in GridCoord::coords(&map) {
        if !visited.insert(start) {
            continue;
        }
        let mut area = 0;
        let mut corners = 0;

        let mut bfs = vec![start];
        while let Some(next) = bfs.pop() {
            area += 1;

            let s = map[next];

            let nb0 = *map
                .get(next.0 + 1)
                .and_then(|r| r.get(next.1))
                .unwrap_or(&'#');
            let nb1 = *map
                .get(next.0)
                .and_then(|r| r.get(next.1 + 1))
                .unwrap_or(&'#');
            let nb2 = *map
                .get(next.0.wrapping_sub(1))
                .and_then(|r| r.get(next.1))
                .unwrap_or(&'#');
            let nb3 = *map
                .get(next.0)
                .and_then(|r| r.get(next.1.wrapping_sub(1)))
                .unwrap_or(&'#');

            let nb01 = *map
                .get(next.0 + 1)
                .and_then(|r| r.get(next.1 + 1))
                .unwrap_or(&'#');
            let nb12 = *map
                .get(next.0.wrapping_sub(1))
                .and_then(|r| r.get(next.1 + 1))
                .unwrap_or(&'#');
            let nb23 = *map
                .get(next.0.wrapping_sub(1))
                .and_then(|r| r.get(next.1.wrapping_sub(1)))
                .unwrap_or(&'#');
            let nb03 = *map
                .get(next.0 + 1)
                .and_then(|r| r.get(next.1.wrapping_sub(1)))
                .unwrap_or(&'#');

            // Outer corner checks
            if nb0 != s && nb1 != s {
                corners += 1;
            }
            if nb1 != s && nb2 != s {
                corners += 1;
            }
            if nb2 != s && nb3 != s {
                corners += 1;
            }
            if nb3 != s && nb0 != s {
                corners += 1;
            }

            // Inner corner checks
            if nb0 == s && nb1 == s && nb01 != s {
                corners += 1;
            }
            if nb1 == s && nb2 == s && nb12 != s {
                corners += 1;
            }
            if nb2 == s && nb3 == s && nb23 != s {
                corners += 1;
            }
            if nb0 == s && nb3 == s && nb03 != s {
                corners += 1;
            }

            for nb in next.neighbours_in(&map) {
                if map[nb] != map[next] {
                    continue;
                }
                if !visited.insert(nb) {
                    continue;
                }
                bfs.push(nb);
            }
        }
        total += area * corners;
    }

    total
}

fn parse_input(inp: &str) -> Vec<Vec<char>> {
    inp.lines().map(|line| line.chars().collect()).collect()
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct GridCoord(usize, usize);
impl<T> Index<GridCoord> for Vec<Vec<T>> {
    type Output = T;

    fn index(&self, index: GridCoord) -> &Self::Output {
        &self[index.0][index.1]
    }
}
impl GridCoord {
    fn neighbours_in<T>(&self, vec: &Vec<Vec<T>>) -> impl Iterator<Item = GridCoord> + 'static {
        let mut output = vec![];
        if self.0 != 0 {
            output.push(GridCoord(self.0 - 1, self.1));
        }
        if self.1 != 0 {
            output.push(GridCoord(self.0, self.1 - 1));
        }
        if self.0 + 1 != vec.len() {
            output.push(GridCoord(self.0 + 1, self.1));
        }
        if self.1 + 1 != vec[0].len() {
            output.push(GridCoord(self.0, self.1 + 1));
        }
        output.into_iter()
    }

    fn coords<T>(vec: &Vec<Vec<T>>) -> impl Iterator<Item = GridCoord> + 'static {
        let y_len = vec.len();
        let x_len = vec[0].len();
        (0..y_len)
            .cartesian_product(0..x_len)
            .map(|(y, x)| GridCoord(y, x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(140, result);
    }

    #[test]
    fn test_part1_ex2() {
        let result = part1(include_str!("example2"));
        assert_eq!(772, result);
    }

    #[test]
    fn test_part1_ex3() {
        let result = part1(include_str!("example3"));
        assert_eq!(1930, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(1485656, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"));
        assert_eq!(80, result);
    }

    #[test]
    fn test_part2_ex2() {
        let result = part2(include_str!("example2"));
        assert_eq!(436, result);
    }

    #[test]
    fn test_part2_ex3() {
        let result = part2(include_str!("example3"));
        assert_eq!(1206, result);
    }

    #[test]
    fn test_part2_ex4() {
        let result = part2(include_str!("example4"));
        assert_eq!(236, result);
    }

    #[test]
    fn test_part2_ex5() {
        let result = part2(include_str!("example5"));
        assert_eq!(368, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(899196, result);
    }
}
