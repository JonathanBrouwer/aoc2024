use itertools::Itertools;

struct Computah {
    a: usize,
    b: usize,
    c: usize,
    ops: Vec<u8>,
}

impl Computah {
    pub fn run(&mut self) -> Vec<u8> {
        let mut pc = 0;
        let mut output = vec![];

        while pc < self.ops.len() {
            let lit = self.ops[pc + 1] as usize;
            let combo = match lit {
                v @ 0..=3 => v,
                4 => self.a,
                5 => self.b,
                6 => self.c,
                7 => 0,
                _ => unreachable!(),
            };

            match self.ops[pc] {
                // ADV
                0 => {
                    self.a >>= combo;
                    pc += 2;
                }
                // BXL
                1 => {
                    self.b ^= lit;
                    pc += 2;
                }
                // BST
                2 => {
                    self.b = combo % 8;
                    pc += 2;
                }
                // JNZ
                3 => {
                    if self.a != 0 {
                        pc = lit;
                    } else {
                        pc += 2;
                    }
                }
                // BXC
                4 => {
                    self.b = self.b ^ self.c;
                    pc += 2;
                }
                // OUT
                5 => {
                    output.push((combo % 8) as u8);
                    pc += 2;
                }
                // BDV
                6 => {
                    self.b = self.a >> combo;
                    pc += 2;
                }
                // CDV
                7 => {
                    self.c = self.a >> combo;
                    pc += 2;
                }
                _ => unreachable!(),
            }
        }

        output
    }

    pub fn run_with_target(&mut self) -> bool {
        let mut pc = 0;
        let mut out_i = 0;

        while pc < self.ops.len() {
            let lit = self.ops[pc + 1] as usize;
            let combo = match lit {
                v @ 0..=3 => v,
                4 => self.a,
                5 => self.b,
                6 => self.c,
                7 => 0,
                _ => unreachable!(),
            };

            match self.ops[pc] {
                // ADV
                0 => {
                    self.a >>= combo;
                    pc += 2;
                }
                // BXL
                1 => {
                    self.b ^= lit;
                    pc += 2;
                }
                // BST
                2 => {
                    self.b = combo % 8;
                    pc += 2;
                }
                // JNZ
                3 => {
                    if self.a != 0 {
                        pc = lit;
                    } else {
                        pc += 2;
                    }
                }
                // BXC
                4 => {
                    self.b = self.b ^ self.c;
                    pc += 2;
                }
                // OUT
                5 => {
                    let result = (combo % 8) as u8;
                    if result != self.ops[out_i] {
                        return false;
                    }
                    out_i += 1;
                    pc += 2;
                }
                // BDV
                6 => {
                    self.b = self.a >> combo;
                    pc += 2;
                }
                // CDV
                7 => {
                    self.c = self.a >> combo;
                    pc += 2;
                }
                _ => unreachable!(),
            }
        }

        out_i == self.ops.len()
    }
}

fn part1(inp: &str) -> String {
    let (a, b, c, ops) = parse_input(inp);
    let mut computer = Computah { a, b, c, ops };
    computer.run().into_iter().map(|v| v.to_string()).join(",")
}

fn part2(inp: &str) -> usize {
    let (_, b, c, ops) = parse_input(inp);
    let iter = ops.iter().copied().rev();

    part2_rec(0, iter, 0).unwrap()
}

fn part2_rec(a: usize, mut ops: impl Iterator<Item = u8> + Clone, i: usize) -> Option<usize> {
    let Some(op) = ops.next() else {
        return Some(a);
    };

    let op = op as usize;

    let target = op ^ 6;

    for x in 0..8 {
        let other = (((a << 3) | x) >> (x ^ 3)) % 8;
        let xo = x ^ other;

        if xo == target {
            let new_a = (a << 3) | x;
            if let Some(v) = part2_rec(new_a, ops.clone(), i + 1) {
                return Some(v);
            }
        }
    }

    None
}

fn parse_input(inp: &str) -> (usize, usize, usize, Vec<u8>) {
    let (regs, program) = inp.split_once("\n\n").unwrap();
    let mut lines = regs.lines();
    let a = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse()
        .unwrap();
    let b = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse()
        .unwrap();
    let c = lines
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse()
        .unwrap();
    let ops = program
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect();
    (a, b, c, ops)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!("4,6,3,5,6,3,5,2,1,0", result);
    }

    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!("1,7,6,5,1,0,5,0,7", result);
    }

    #[test]
    fn test_part2_real() {
        let inp = include_str!("input");
        let (_, b, c, ops) = parse_input(inp);
        let result = part2(inp);
        println!("Part 2: {}", result);

        let (regs, program) = inp.split_once("\n\n").unwrap();
        let mut computer = Computah {
            a: result,
            b,
            c,
            ops: ops.clone(),
        };
        assert_eq!(computer.run(), ops);
        assert_eq!(result, 236555995274861);
    }
}
