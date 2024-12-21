use itertools::Itertools;
use std::collections::VecDeque;

fn part1(inp: &str, at_least: usize) -> usize {
    let (map, (_sy, _sx), (ey, ex)) = parse_input(inp);

    let mut distances_from_end = vec![vec![usize::MAX; map[0].len()]; map.len()];
    let mut queue = VecDeque::new();
    queue.push_front((ey, ex, 0));

    while let Some((ny, nx, distance)) = queue.pop_back() {
        if map[ny][nx] == '#' {
            continue;
        }
        if distances_from_end[ny][nx] != usize::MAX {
            continue;
        }
        distances_from_end[ny][nx] = distance;
        queue.push_front((ny + 1, nx, distance + 1));
        queue.push_front((ny - 1, nx, distance + 1));
        queue.push_front((ny, nx + 1, distance + 1));
        queue.push_front((ny, nx - 1, distance + 1));
    }

    let mut count = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '#' {
                continue;
            }
            let cheat_positions = [
                (y + 2, x),
                (y.wrapping_sub(2), x),
                (y, x + 2),
                (y, x.wrapping_sub(2)),
                (y + 1, x + 1),
                (y + 1, x - 1),
                (y - 1, x + 1),
                (y - 1, x - 1),
            ];
            for (cy, cx) in cheat_positions {
                if cy >= map.len() || cx >= map[0].len() {
                    continue;
                }
                if map[cy][cx] == '#' {
                    continue;
                }
                if distances_from_end[cy][cx] + at_least + 2 <= distances_from_end[y][x] {
                    count += 1;
                }
            }
        }
    }

    count
}

fn part2(inp: &str, at_least: usize) -> usize {
    let (map, (_sy, _sx), (ey, ex)) = parse_input(inp);

    let mut distances_from_end = vec![vec![usize::MAX; map[0].len()]; map.len()];
    let mut queue = VecDeque::new();
    queue.push_front((ey, ex, 0));

    while let Some((ny, nx, distance)) = queue.pop_back() {
        if map[ny][nx] == '#' {
            continue;
        }
        if distances_from_end[ny][nx] != usize::MAX {
            continue;
        }
        distances_from_end[ny][nx] = distance;
        queue.push_front((ny + 1, nx, distance + 1));
        queue.push_front((ny - 1, nx, distance + 1));
        queue.push_front((ny, nx + 1, distance + 1));
        queue.push_front((ny, nx - 1, distance + 1));
    }

    let mut count = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '#' {
                continue;
            }
            let mut cheat_positions = Vec::new();
            for dy in 0..=20 {
                for dx in 0..=(20 - dy) {
                    if dy != 0 {
                        cheat_positions.push((y.wrapping_sub(dy), x.wrapping_sub(dx), dy + dx));
                        if dx != 0 {
                            cheat_positions.push((y.wrapping_sub(dy), x.wrapping_add(dx), dy + dx));
                        }
                    }
                    cheat_positions.push((y.wrapping_add(dy), x.wrapping_add(dx), dy + dx));
                    if dx != 0 {
                        cheat_positions.push((y.wrapping_add(dy), x.wrapping_sub(dx), dy + dx));
                    }
                }
            }

            for (cy, cx, d) in cheat_positions {
                if cy >= map.len() || cx >= map[0].len() {
                    continue;
                }
                if map[cy][cx] == '#' {
                    continue;
                }
                if distances_from_end[cy][cx] + at_least + d <= distances_from_end[y][x] {
                    count += 1;
                }
            }
        }
    }

    count
}

fn parse_input(inp: &str) -> (Vec<Vec<char>>, (usize, usize), (usize, usize)) {
    let map = inp
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let start = map
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .find(|(_, c)| **c == 'S')
                .map(|(x, _)| (y, x))
        })
        .next()
        .unwrap();
    let end = map
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .find(|(_, c)| **c == 'E')
                .map(|(x, _)| (y, x))
        })
        .next()
        .unwrap();
    (map, start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"), 10);
        assert_eq!(10, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"), 100);
        println!("Part 1: {}", result);
        assert_eq!(1406, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"), 50);
        assert_eq!(285, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"), 100);
        println!("Part 2: {}", result);
        assert_eq!(1006101, result);
    }
}
