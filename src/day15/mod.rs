use itertools::Itertools;
use std::fmt::{Display, Formatter};

fn part1(inp: &str) -> usize {
    let (mut map, moves, (mut py, mut px)) = parse_input(inp);
    'outer: for (my, mx) in moves {
        let fy = (py as isize + my) as usize;
        let fx = (px as isize + mx) as usize;

        for i in 1.. {
            let ny = (py as isize + i * my) as usize;
            let nx = (px as isize + i * mx) as usize;
            match map[ny][nx] {
                Tile::Box => continue,
                Tile::Wall => continue 'outer,
                Tile::Empty => {
                    if map[fy][fx] == Tile::Box {
                        map[fy][fx] = Tile::Empty;
                        map[ny][nx] = Tile::Box;
                    }
                    py = fy;
                    px = fx;
                    continue 'outer;
                }
            }
        }
    }

    let mut sum = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == Tile::Box {
                sum += 100 * y + x;
            }
        }
    }

    sum
}

fn add_box(
    queue: &mut Vec<(usize, usize)>,
    pos: (usize, usize),
    map: &Vec<Vec<LargeTile>>,
    my: isize,
) {
    match map[pos.0][pos.1] {
        LargeTile::Wall | LargeTile::Empty => {
            queue.push(pos);
        }
        LargeTile::Box(side) => {
            queue.push(pos);
            if my != 0 {
                match side {
                    BoxSide::Left => queue.push((pos.0, pos.1 + 1)),
                    BoxSide::Right => queue.push((pos.0, pos.1 - 1)),
                }
            }
        }
    }
}

fn display_map(map: &Vec<Vec<LargeTile>>, py: usize, px: usize) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if y == py && x == px {
                print!("@")
            } else {
                print!("{}", map[y][x])
            }
        }
        println!()
    }
}

fn part2(inp: &str) -> usize {
    let (map, moves, (mut py, mut px)) = parse_input(inp);
    px *= 2;
    let mut map = map
        .into_iter()
        .map(|row| {
            row.into_iter()
                .flat_map(|v| match v {
                    Tile::Box => [
                        LargeTile::Box(BoxSide::Left),
                        LargeTile::Box(BoxSide::Right),
                    ],
                    Tile::Wall => [LargeTile::Wall, LargeTile::Wall],
                    Tile::Empty => [LargeTile::Empty, LargeTile::Empty],
                })
                .collect_vec()
        })
        .collect_vec();

    'outer: for (my, mx) in moves {
        let fy = (py as isize + my) as usize;
        let fx = (px as isize + mx) as usize;
        let mut queue = vec![];
        add_box(&mut queue, (fy, fx), &map, my);

        'inner: for i in 0.. {
            let Some(&(ny, nx)) = queue.get(i) else { break };
            match map[ny][nx] {
                LargeTile::Wall => continue 'outer,
                LargeTile::Empty => continue 'inner,
                LargeTile::Box(_) => {
                    let nny = (ny as isize + my) as usize;
                    let nnx = (nx as isize + mx) as usize;
                    add_box(&mut queue, (nny, nnx), &map, my);
                }
            }
        }

        for (y, x) in queue.into_iter().rev() {
            if map[y][x] == LargeTile::Empty {
                continue;
            }
            map[(y as isize + my) as usize][(x as isize + mx) as usize] = map[y][x];
            map[y][x] = LargeTile::Empty;
        }

        py = fy;
        px = fx;
    }

    let mut sum = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == LargeTile::Box(BoxSide::Left) {
                sum += 100 * y + x;
            }
        }
    }

    sum
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum LargeTile {
    Wall,
    Box(BoxSide),
    Empty,
}

impl Display for LargeTile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LargeTile::Wall => '#',
                LargeTile::Box(BoxSide::Left) => '[',
                LargeTile::Box(BoxSide::Right) => ']',
                LargeTile::Empty => '.',
            }
        )
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum BoxSide {
    Left,
    Right,
}

#[derive(Eq, PartialEq)]
enum Tile {
    Wall,
    Box,
    Empty,
}

fn parse_input(inp: &str) -> (Vec<Vec<Tile>>, Vec<(isize, isize)>, (usize, usize)) {
    let (map, moves) = inp.split_once("\n\n").unwrap();

    let mut start_pos = (0usize, 0usize);
    let map = map
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => Tile::Wall,
                    'O' => Tile::Box,
                    '.' => Tile::Empty,
                    '@' => {
                        start_pos = (y, x);
                        Tile::Empty
                    }
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .collect_vec();

    let moves = moves
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| match c {
            '<' => (0, -1),
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            _ => unreachable!(),
        })
        .collect_vec();

    (map, moves, start_pos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(10092, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(1516281, result);
    }

    #[test]
    fn test_part2_ex2() {
        let result = part2(include_str!("example2"));
        assert_eq!(618, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(1527969, result);
    }
}
