use std::collections::VecDeque;

fn part1(inp: &str) -> usize {
    let mut state = VecDeque::new();
    let mut i = 0;
    let mut s = true;
    for c in inp.chars() {
        let c = c.to_digit(10).unwrap();
        if s {
            for _ in 0..c {
                state.push_back(Some(i))
            }
            i += 1;
        } else {
            for _ in 0..c {
                state.push_back(None)
            }
        }
        s = !s;
    }

    let mut sum = 0;
    let mut i = 0;
    'outer: while let Some(next) = state.pop_front() {
        let v = if let Some(next) = next {
            next
        } else {
            loop {
                let Some(back) = state.pop_back() else {
                    break 'outer;
                };
                if let Some(back) = back {
                    break back;
                }
            }
        };
        sum += i * v;
        i += 1;
    }

    sum
}

fn part2(inp: &str) -> usize {
    #[derive(Debug)]
    enum State {
        File(usize, usize),
        Empty(usize),
    }
    let mut state: Vec<State> = Vec::new();

    let mut i = 0;
    let mut c = true;
    for digit in inp.chars().map(|c| c.to_digit(10).unwrap()) {
        if c {
            state.push(State::File(i, digit as usize));
            i += 1;
        } else {
            state.push(State::Empty(digit as usize));
        }
        c = !c;
    }

    let mut right = state.len();
    loop {
        let Some(new_right) = right.checked_sub(1) else {
            break;
        };
        right = new_right;

        let &State::File(i, l_needed) = &state[right] else {
            continue;
        };

        // for state in &state {
        //     match state {
        //         State::File(i, c) => for _ in 0..*c {
        //             print!("{i}")
        //         }
        //         State::Empty(c) => for _ in 0..*c {
        //             print!(".")
        //         }
        //     }
        // }
        // println!();

        for left in 0..right {
            let State::Empty(l_available) = &mut state[left] else {
                continue;
            };
            if *l_available < l_needed {
                continue;
            }
            *l_available -= l_needed;
            state[right] = State::Empty(l_needed);
            state.insert(left, State::File(i, l_needed));
            break;
        }
    }

    let mut sum = 0usize;
    let mut i = 0usize;
    for state in state {
        match state {
            State::File(j, l) => {
                for k in i..i + l {
                    sum += j * k;
                }
                i += l;
            }
            State::Empty(c) => i += c,
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(1928, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(6334655979668, result);
    }

    #[test]
    fn test_part2_ex1() {
        let result = part2(include_str!("example1"));
        assert_eq!(2858, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!(6349492251099, result);
    }
}
