fn part1(inp: &str) -> usize {
    let field = parse_input(inp);
    const DIRS: [(usize, usize); 8] = [
        (1, 0),
        (1, 1),
        (0, 1),
        (usize::MAX, 1),
        (usize::MAX, 0),
        (usize::MAX, usize::MAX),
        (0, usize::MAX),
        (1, usize::MAX),
    ];

    let mut count = 0;
    for y in 0..field.len() {
        for x in 0..field[y].len() {
            for (dy, dx) in DIRS {
                if "XMAS"
                    .chars()
                    .enumerate()
                    .all(|(i, c)| get(&field, add((y, x), i, (dy, dx))) == Some(c))
                {
                    count += 1;
                }
            }
        }
    }

    count
}

fn add(pos: (usize, usize), times: usize, dir: (usize, usize)) -> (usize, usize) {
    (
        pos.0.wrapping_add(dir.0.wrapping_mul(times)),
        pos.1.wrapping_add(dir.1.wrapping_mul(times)),
    )
}

fn get(field: &[Vec<char>], pos: (usize, usize)) -> Option<char> {
    field.get(pos.0)?.get(pos.1).copied()
}

fn part2(inp: &str) -> usize {
    let field = parse_input(inp);

    let mut count = 0;
    for y in 0..field.len() - 2 {
        for x in 0..field[y].len() - 2 {
            if field[y + 1][x + 1] == 'A'
                && is_ms_pair(field[y][x], field[y + 2][x + 2])
                && is_ms_pair(field[y + 2][x], field[y][x + 2])
            {
                count += 1;
            }
        }
    }

    count
}

fn is_ms_pair(a: char, b: char) -> bool {
    matches!((a, b), ('M', 'S') | ('S', 'M'))
}

fn parse_input(inp: &str) -> Vec<Vec<char>> {
    inp.lines().map(|l| l.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(18, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(2454, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"));
        assert_eq!(9, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(1858, result);
    }
}
