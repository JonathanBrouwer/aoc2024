use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, VecDeque};

struct State {
    pos: (usize, usize),
    score: usize,
}
impl Eq for State {}
impl PartialEq<Self> for State {
    fn eq(&self, other: &Self) -> bool {
        self.score.eq(&other.score)
    }
}
impl PartialOrd<Self> for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.score.cmp(&other.score).reverse())
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score).reverse()
    }
}

fn part1(inp: &str, size: usize, cap: usize) -> usize {
    let start = (0, 0);
    let end = (size - 1, size - 1);

    let mut map = vec![vec![false; size]; size];
    for line in inp.lines().take(cap) {
        let (x, y) = line.split_once(",").unwrap();
        map[y.parse::<usize>().unwrap()][x.parse::<usize>().unwrap()] = true;
    }

    let mut heap = VecDeque::new();
    heap.push_front(State {
        pos: start,
        score: 0,
    });
    let mut distances = HashMap::new();

    while let Some(next) = heap.pop_back() {
        if next.pos == end {
            return next.score;
        }
        if map[next.pos.0][next.pos.1] {
            continue;
        }
        if distances.contains_key(&next.pos) {
            continue;
        }
        distances.insert(next.pos, next.score);

        for (dy, dx) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
            let ny = ((next.pos.0 as isize) + dy) as usize;
            let nx = ((next.pos.1 as isize) + dx) as usize;

            if ny < size && nx < size {
                heap.push_front(State {
                    pos: (ny, nx),
                    score: next.score + 1,
                })
            }
        }
    }
    unreachable!()
}

fn part2(inp: &str, size: usize) -> String {
    let start = (0, 0);
    let end = (size - 1, size - 1);

    let mut map = vec![vec![false; size]; size];
    'line: for line in inp.lines() {
        let (x, y) = line.split_once(",").unwrap();
        map[y.parse::<usize>().unwrap()][x.parse::<usize>().unwrap()] = true;

        let mut heap = VecDeque::new();
        heap.push_front(State {
            pos: start,
            score: 0,
        });
        let mut distances = HashMap::new();

        while let Some(next) = heap.pop_back() {
            if next.pos == end {
                continue 'line;
            }
            if map[next.pos.0][next.pos.1] {
                continue;
            }
            if distances.contains_key(&next.pos) {
                continue;
            }
            distances.insert(next.pos, next.score);

            for (dy, dx) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
                let ny = ((next.pos.0 as isize) + dy) as usize;
                let nx = ((next.pos.1 as isize) + dx) as usize;

                if ny < size && nx < size {
                    heap.push_front(State {
                        pos: (ny, nx),
                        score: next.score + 1,
                    })
                }
            }
        }

        return format!("{x},{y}");
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"), 7, 12);
        assert_eq!(22, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"), 71, 1024);
        println!("Part 1: {}", result);
        assert_eq!(262, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"), 7);
        assert_eq!("6,1", result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"), 71);
        println!("Part 2: {}", result);
        assert_eq!("22,20", result);
    }
}
