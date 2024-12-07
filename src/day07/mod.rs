fn part1(inp: &str) -> usize {
    parse_input(inp).into_iter().filter(|(goal, nums)| {
        nums.iter().rev().fold(vec![*goal], |possibilities, &next| {
            let mut new = vec![];
            for pos in possibilities {
                //mul
                if pos % next == 0 {
                    new.push(pos / next);
                }
                //add
                if let Some(r) = pos.checked_sub(next) {
                    new.push(r);
                }
            }
            new
        }).contains(&0)
    }).map(|(goal, _)| goal).sum()
}

fn part2(inp: &str) -> usize {
    parse_input(inp).into_iter().filter(|(goal, nums)| {
        nums.iter().rev().fold(vec![*goal], |possibilities, &next| {
            let mut new = vec![];
            for pos in possibilities {
                //mul
                if pos % next == 0 {
                    new.push(pos / next);
                }
                //add
                if let Some(r) = pos.checked_sub(next) {
                    new.push(r);
                }
                //concat
                let mask = 10usize.pow(next.ilog10() + 1);
                if pos % mask == next {
                    new.push(pos / mask);
                }
            }
            new
        }).contains(&0)
    }).map(|(goal, _)| goal).sum()
}

fn parse_input(inp: &str) -> Vec<(usize, Vec<usize>)> {
    inp.lines().map(|line| {
    let (goal, nums) = line.split_once(": ").unwrap();
    (goal.parse().unwrap(), nums.split(" ").map(|n| n.parse().unwrap()).collect())
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(3749, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(7579994664753, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"));
        assert_eq!(11387, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(438027111276610, result);
    }
}
