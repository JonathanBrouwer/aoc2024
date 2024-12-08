use std::collections::HashSet;

fn part1(inp: &str) -> usize {
    let (map, (mut py, mut px)) = parse_input(inp);
    let mut dir = (-1isize, 0isize);
    let mut visited = HashSet::new();

    loop {
        visited.insert((py, px));

        let py2 = (py as isize + dir.0) as usize;
        let px2 = (px as isize + dir.1) as usize;
        if let Some(v) = map.get(py2).and_then(|row| row.get(px2)) {
            if *v {
                dir = match dir {
                    (-1, 0) => (0, 1),
                    (0, 1) => (1, 0),
                    (1, 0) => (0, -1),
                    (0, -1) => (-1, 0),
                    _ => unreachable!(),
                }
            } else {
                py = py2;
                px = px2;
            }
        } else {
            break;
        }
    }

    visited.len()
}

fn part2(inp: &str) -> usize {
    let (mut map, (py, px)) = parse_input(inp);

    let mut count = 0;
    for ty in 0..map.len() {
        for tx in 0..map[0].len() {
            if map[ty][tx] {
                continue;
            }
            if ty == py && tx == px {
                continue;
            }
            map[ty][tx] = true;

            let mut py = py;
            let mut px = px;
            let mut dir = (-1isize, 0isize);
            let mut visited = HashSet::new();

            loop {
                if !visited.insert((py, px, dir)) {
                    count += 1;
                    break;
                }

                let py2 = (py as isize + dir.0) as usize;
                let px2 = (px as isize + dir.1) as usize;
                if let Some(v) = map.get(py2).and_then(|row| row.get(px2)) {
                    if *v {
                        dir = match dir {
                            (-1, 0) => (0, 1),
                            (0, 1) => (1, 0),
                            (1, 0) => (0, -1),
                            (0, -1) => (-1, 0),
                            _ => unreachable!(),
                        }
                    } else {
                        py = py2;
                        px = px2;
                    }
                } else {
                    break;
                }
            }

            map[ty][tx] = false;
        }
    }

    count
}

/// true = wall
fn parse_input(inp: &str) -> (Vec<Vec<bool>>, (usize, usize)) {
    let pos = inp
        .lines()
        .enumerate()
        .find_map(|(y, l)| {
            l.chars()
                .enumerate()
                .find(|c| c.1 == '^')
                .map(|(x, _c)| (y, x))
        })
        .unwrap();
    (
        inp.lines()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect(),
        pos,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(41, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(5242, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"));
        assert_eq!(6, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(1424, result);
    }
}
