use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn turn_right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn turn_left(self) -> Self {
        self.turn_right().turn_right().turn_right()
    }

    pub fn delta(self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Position {
    y: usize,
    x: usize,
    dir: Direction,
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct State {
    pos: Position,
    score: usize,
}

impl State {
    fn neighbours(self, map: &Vec<Vec<char>>) -> Vec<State> {
        let mut states = vec![];

        // Turn left, right
        for dir in [self.pos.dir.turn_left(), self.pos.dir.turn_right()] {
            states.push(State {
                pos: Position { dir, ..self.pos },
                score: self.score + 1000,
            })
        }

        // Move forward
        let (dy, dx) = self.pos.dir.delta();

        let y = (self.pos.y as isize + dy) as usize;
        let x = (self.pos.x as isize + dx) as usize;
        if map[y][x] != '#' {
            states.push(State {
                pos: Position { y, x, ..self.pos },
                score: self.score + 1,
            });
        }

        states
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score).reverse()
    }
}

fn part1(inp: &str) -> usize {
    let (map, (start_y, start_x), (end_y, end_x)) = parse_input(inp);
    let mut heap = BinaryHeap::new();
    let mut distances = HashMap::new();

    let start = Position {
        y: start_y,
        x: start_x,
        dir: Direction::East,
    };
    heap.push(State {
        pos: start,
        score: 0,
    });

    while let Some(next) = heap.pop() {
        if next.pos.y == end_y && next.pos.x == end_x {
            return next.score;
        }
        if distances.contains_key(&next.pos) {
            continue;
        }
        distances.insert(next.pos, next.score);

        heap.extend(next.neighbours(&map));
    }

    unreachable!()
}

fn part2(inp: &str) -> usize {
    let (map, (start_y, start_x), (end_y, end_x)) = parse_input(inp);
    let mut heap = BinaryHeap::new();
    let mut distances = HashMap::new();

    let start = Position {
        y: start_y,
        x: start_x,
        dir: Direction::East,
    };
    heap.push(State {
        pos: start,
        score: 0,
    });

    let mut end_state = None;
    while let Some(next) = heap.pop() {
        if distances.contains_key(&next.pos) {
            continue;
        }
        distances.insert(next.pos, next.score);
        if next.pos.y == end_y && next.pos.x == end_x {
            end_state = Some(next);
            break;
        }

        heap.extend(next.neighbours(&map));
    }

    let end_state = end_state.unwrap();
    let mut queue = vec![end_state];
    let mut positions = HashSet::new();
    while let Some(mut next) = queue.pop() {
        if let Some(d) = distances.remove(&next.pos) {
            if d != next.score {
                continue;
            }
            positions.insert((next.pos.y, next.pos.x));
        } else {
            continue;
        }

        next.pos.dir = next.pos.dir.turn_left().turn_left();
        for mut nb in next.neighbours(&map) {
            nb.pos.dir = nb.pos.dir.turn_left().turn_left();
            let Some(nb_score) = (2 * next.score).checked_sub(nb.score) else {
                continue;
            };
            nb.score = nb_score;

            queue.push(nb);
        }
    }
    positions.len()
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
        let result = part1(include_str!("example1"));
        assert_eq!(7036, result);
    }

    #[test]
    fn test_part1_ex2() {
        let result = part1(include_str!("example2"));
        assert_eq!(11048, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(109516, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"));
        assert_eq!(45, result);
    }

    #[test]
    fn test_part2_ex2() {
        let result = part2(include_str!("example2"));
        assert_eq!(64, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(568, result);
    }
}
