use itertools::Itertools;
use scan_fmt::scan_fmt;
use std::collections::HashSet;

fn part1(inp: &str, size_x: isize, size_y: isize) -> usize {
    const STEPS: isize = 100;

    let mut q0 = 0;
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;

    for (px, py, vx, vy) in parse_input(inp) {
        let final_x = (px + vx * STEPS).rem_euclid(size_x);
        let final_y = (py + vy * STEPS).rem_euclid(size_y);

        if final_x < size_x / 2 {
            if final_y < size_y / 2 {
                q0 += 1;
            } else if final_y > size_y / 2 {
                q1 += 1;
            }
        } else if final_x > size_x / 2 {
            if final_y < size_y / 2 {
                q2 += 1;
            } else if final_y > size_y / 2 {
                q3 += 1;
            }
        }
    }

    q0 * q1 * q2 * q3
}

fn part2(inp: &str, size_x: usize, size_y: usize) -> usize {
    let input = parse_input(inp).collect_vec();

    for seconds in 0.. {
        let mut field = vec![vec![false; size_x]; size_y];
        for &(px, py, vx, vy) in &input {
            let final_x = (px + vx * seconds).rem_euclid(size_x as isize) as usize;
            let final_y = (py + vy * seconds).rem_euclid(size_y as isize) as usize;
            field[final_y][final_x] = true;
        }

        let mut found = HashSet::new();
        let mut max_blob = 0;
        for &(px, py, vx, vy) in &input {
            let final_x = (px + vx * seconds).rem_euclid(size_x as isize) as usize;
            let final_y = (py + vy * seconds).rem_euclid(size_y as isize) as usize;

            let mut size_blob = 0;
            let mut todo = vec![(final_y, final_x)];
            while let Some((ny, nx)) = todo.pop() {
                size_blob += 1;
                for (nby, nbx) in [
                    ((ny + 1) % size_y, nx),
                    ((ny + size_y - 1) % size_y, nx),
                    (ny, (nx + 1) % size_x),
                    (ny, (nx + size_x - 1) % size_x),
                ] {
                    if !field[nby][nbx] {
                        continue;
                    }
                    if !found.insert((nby, nbx)) {
                        continue;
                    }
                    todo.push((nby, nbx));
                }
            }
            max_blob = max_blob.max(size_blob);
        }

        if max_blob > input.len() / 10 {
            for y in 0..size_y {
                for x in 0..size_x {
                    if field[y][x] {
                        print!("#")
                    } else {
                        print!(".")
                    }
                }
                println!()
            }
            return seconds as usize;
        }
    }

    0
}

fn parse_input(inp: &str) -> impl Iterator<Item = (isize, isize, isize, isize)> + use<'_> {
    inp.lines()
        .map(|line| scan_fmt!(line, "p={},{} v={},{}", isize, isize, isize, isize).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"), 11, 7);
        assert_eq!(12, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"), 101, 103);
        println!("Part 1: {}", result);
        assert_eq!(217328832, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"), 101, 103);
        println!("Part 2: {}", result);
        assert_eq!(7412, result);
    }
}
