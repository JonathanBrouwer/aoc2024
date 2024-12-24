use itertools::Itertools;
use std::collections::HashMap;

fn part1(inp: &str) -> usize {
    let (inputs, gates) = parse_input(inp);
    let gate_inputs = gates
        .iter()
        .enumerate()
        .flat_map(|(i, (a, _, b, _))| [(a, i), (b, i)])
        .into_group_map();
    let mut wire_values = HashMap::new();
    let mut queue = vec![];

    for (input, v) in inputs {
        wire_values.insert(input, v);
        queue.extend(gate_inputs.get(&input).unwrap().iter().copied());
    }

    while let Some(next) = queue.pop() {
        let (left, gate, right, out) = &gates[next];
        let Some(&left) = wire_values.get(left) else {
            continue;
        };
        let Some(&right) = wire_values.get(right) else {
            continue;
        };
        wire_values.insert(
            out,
            match *gate {
                "AND" => left && right,
                "OR" => left || right,
                "XOR" => left ^ right,
                _ => unreachable!(),
            },
        );
        if let Some(next) = gate_inputs.get(&out) {
            queue.extend(next.iter().copied());
        }
    }

    wire_values
        .iter()
        .filter(|(k, _)| k.starts_with("z"))
        .sorted_by_key(|(k, _v)| *k)
        .rev()
        .fold(0, |cur, (_, next)| (cur << 1) | (*next as usize))
}

fn part2(inp: &str) -> String {
    let (_inputs, mut gates) = parse_input(inp);

    let mut swaps: Vec<(&str, &str)> = vec![];

    'outer: loop {
        let mut carry_in = None;
        for i in 0..44 {
            let Some(xor_gate) = find_gate(
                &format!("x{i:02}"),
                &format!("y{i:02}"),
                "XOR",
                &mut swaps,
                &mut gates,
            ) else {
                continue 'outer;
            };
            let Some(and_gate) = find_gate(
                &format!("x{i:02}"),
                &format!("y{i:02}"),
                "AND",
                &mut swaps,
                &mut gates,
            ) else {
                continue 'outer;
            };
            let Some(carry_in) = &mut carry_in else {
                carry_in = Some(and_gate);
                continue;
            };

            let Some(xor_gate2) = find_gate(xor_gate, carry_in, "XOR", &mut swaps, &mut gates)
            else {
                continue 'outer;
            };
            if assert_equal_or_swap(xor_gate2, &format!("z{i:02}"), &mut swaps, &mut gates) {
                continue 'outer;
            }

            let Some(and_gate2) = find_gate(xor_gate, carry_in, "AND", &mut swaps, &mut gates)
            else {
                continue 'outer;
            };
            let Some(or_gate) = find_gate(and_gate2, and_gate, "OR", &mut swaps, &mut gates) else {
                continue 'outer;
            };
            *carry_in = or_gate;
        }
        break;
    }
    swaps.iter().flat_map(|(a, b)| [*a, *b]).sorted().join(",")
}

fn assert_equal_or_swap<'a>(
    a: &str,
    b: &str,
    swaps: &mut Vec<(&'a str, &'a str)>,
    gates: &mut Vec<(&'a str, &'a str, &'a str, &'a str)>,
) -> bool {
    if a == b {
        return false;
    }
    let a = gates.iter().find(|(_, _, _, d)| *d == a).unwrap().3;
    let b = gates.iter().find(|(_, _, _, d)| *d == b).unwrap().3;
    swaps.push((a, b));

    for (_, _, _, out) in gates {
        if *out == a {
            *out = b;
        } else if *out == b {
            *out = a;
        }
    }
    return true;
}

fn find_gate<'a>(
    in1: &str,
    in2: &str,
    typ: &str,
    swaps: &mut Vec<(&'a str, &'a str)>,
    gates: &mut Vec<(&'a str, &'a str, &'a str, &'a str)>,
) -> Option<&'a str> {
    if let Some(out) = gates
        .iter()
        .filter(|&&(a, b, c, d)| b == typ && (a == in1 && c == in2 || a == in2 && c == in1))
        .exactly_one()
        .ok()
    {
        return Some(out.3);
    }

    // Find a gate of the right type and that has one input (approximation)
    let Some((a, b, c, d)) = gates
        .iter()
        .filter(|&&(a, b, c, d)| b == typ && (a == in1 || c == in2 || a == in2 || c == in1))
        .exactly_one()
        .ok()
    else {
        panic!("AAAAAAAA")
    };
    let d = *d;

    let (sa, sb) = if *a == in1 {
        (*c, in2)
    } else if *a == in2 {
        (*c, in1)
    } else if *c == in1 {
        (*a, in2)
    } else if *c == in2 {
        (*a, in1)
    } else {
        unreachable!()
    };
    let sb = gates.iter().find(|(_, _, _, d)| *d == sb).unwrap().3;

    swaps.push((sa, sb));

    for (_, _, _, out) in gates {
        if *out == sa {
            *out = sb;
        } else if *out == sb {
            *out = sa;
        }
    }

    None
}

fn parse_input(inp: &str) -> (Vec<(&str, bool)>, Vec<(&str, &str, &str, &str)>) {
    let (inputs, gates) = inp.split_once("\n\n").unwrap();
    let inputs = inputs
        .lines()
        .map(|line| {
            let (g, v) = line.split_once(": ").unwrap();
            (g, v == "1")
        })
        .collect_vec();

    let gates = gates
        .lines()
        .map(|line| {
            let (a, b, c, _, d) = line.split(" ").next_tuple().unwrap();
            (a, b, c, d)
        })
        .collect_vec();

    (inputs, gates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part1(include_str!("example1"));
        assert_eq!(4, result);
    }

    #[test]
    fn test_part1_ex2() {
        let result = part1(include_str!("example2"));
        assert_eq!(2024, result);
    }
    #[test]
    fn test_part1_real() {
        let result = part1(include_str!("input"));
        println!("Part 1: {}", result);
        assert_eq!(51745744348272, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"));
        println!("Part 2: {}", result);
        assert_eq!("bfq,bng,fjp,hkh,hmt,z18,z27,z31", result);
    }
}
