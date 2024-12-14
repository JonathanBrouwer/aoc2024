use scan_fmt::scan_fmt;

fn part1(inp: &str) -> usize {
    parse_input(inp)
        .map(|(but_a, but_b, target)| solve(but_a, but_b, target))
        .sum()
}

fn part2(inp: &str) -> usize {
    parse_input(inp)
        .map(|(but_a, but_b, target)| {
            solve(
                but_a,
                but_b,
                (target.0 + 10000000000000, target.1 + 10000000000000),
            )
        })
        .sum()
}

fn solve(but_a: (usize, usize), but_b: (usize, usize), target: (usize, usize)) -> usize {
    let a = but_a.0 as isize;
    let b = but_b.0 as isize;
    let c = but_a.1 as isize;
    let d = but_b.1 as isize;

    let det = a * d - b * c;
    let inv_a = d;
    let inv_b = -b;
    let inv_c = -c;
    let inv_d = a;

    let a_presses = inv_a * target.0 as isize + inv_b * target.1 as isize;
    if a_presses % det != 0 {
        return 0;
    }
    let b_presses = inv_c * target.0 as isize + inv_d * target.1 as isize;
    if b_presses % det != 0 {
        return 0;
    }
    ((3 * a_presses + b_presses) / det) as usize
}

fn parse_input(
    inp: &str,
) -> impl Iterator<Item = ((usize, usize), (usize, usize), (usize, usize))> + use<'_> {
    inp.split("\n\n").map(|group| {
        let mut lines = group.lines();

        let a = scan_fmt!(lines.next().unwrap(), "Button A: X+{}, Y+{}", usize, usize).unwrap();
        let b = scan_fmt!(lines.next().unwrap(), "Button B: X+{}, Y+{}", usize, usize).unwrap();
        let t = scan_fmt!(lines.next().unwrap(), "Prize: X={}, Y={}", usize, usize).unwrap();

        (a, b, t)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(480, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(35997, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(82510994362072, result);
    }
}
