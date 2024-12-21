use itertools::Itertools;

const NUMPAD_KEYS: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    ['X', '0', 'A'],
];

fn part2(inp: &str, indirections: usize) -> usize {
    // aurdl
    let mut au = 0;
    let mut ar = 0;
    let mut ad = 0;
    let mut al = 0;

    let mut ua = 0;
    let mut ur = 0;
    let mut ul = 0;

    let mut ra = 0;
    let mut ru = 0;
    let mut rd = 0;

    let mut da = 0;
    let mut dr = 0;
    let mut dl = 0;

    let mut la = 0;
    let mut lu = 0;
    let mut ld = 0;

    for _ in 0..indirections {
        let nau = al + 1 + la;
        let nar = ad + 1 + da;
        let nad = al + 1 + ld + 1 + da;
        let nal = ad + 1 + dl + 2 + la;

        let nua = ar + 1 + ra;
        let nur = ad + 1 + dr + 1 + ra;
        let nul = ad + 1 + dl + 1 + la;

        let nra = au + 1 + ua;
        let nru = al + 1 + lu + 1 + ua;
        let nrd = al + 1 + la;

        let nda = au + 1 + ur + 1 + ra;
        let ndr = ar + 1 + ra;
        let ndl = al + 1 + la;

        let nla = ar + 2 + ru + 1 + ua;
        let nlu = ar + 1 + ru + 1 + ua;
        let nld = ar + 1 + ra;

        au = nau;
        ar = nar;
        ad = nad;
        al = nal;
        ua = nua;
        ur = nur;
        ul = nul;
        ra = nra;
        ru = nru;
        rd = nrd;
        da = nda;
        dr = ndr;
        dl = ndl;
        la = nla;
        lu = nlu;
        ld = nld;

        assert!(al + ld + da <= ad + dl + la);
        assert!(ar + rd + da >= ad + dr + ra);
        assert!(au + ul + la >= al + lu + ua);
        assert!(au + ur + ra <= ar + ru + ua);
    }


    let mut sum = 0;
    for code in parse_input(inp) {
        let mut distance = 0;

        let (mut prev_y, mut prev_x) = (3, 2);
        for digit in &code {
            let (y, x) = NUMPAD_KEYS.iter().enumerate().flat_map(|(y, row)| {
                row.iter().find_position(|v| *v == digit).map(|(x, _)| (y, x))
            }).next().unwrap();

            let old_distance = distance;
            if y < prev_y {
                let up = prev_y - y;
                if x > prev_x {
                    let right = x - prev_x;

                    // up & right
                    assert!(au + ur + ra <= ar + ru + ua);
                    distance += up + au + right + ur + ra + 1;
                } else if x < prev_x {
                    let left = prev_x - x;

                    // up & left
                    distance += up + left + 1 + if prev_y == 3 && x == 0 {
                        au + ul + la
                    } else {
                        assert!(al + lu + ua <= au + ul + la);
                        al + lu + ua
                    };
                } else {
                    // only up
                    distance += up + au + ua + 1;
                }
            } else if y > prev_y {
                let down = y - prev_y;
                if x > prev_x {
                    let right = x - prev_x;

                    // down & right
                    distance += down + right + 1 + if prev_x == 0 && y == 3 {
                        ar + rd + da
                    } else {
                        assert!(ad + dr + ra <= ar + rd + da);
                        ad + dr + ra
                    }
                } else if x < prev_x {
                    let left = prev_x - x;

                    // down & left
                    assert!(ad + dl + la <= al + ld + da);
                    distance += down + ad + left + dl + la + 1;
                } else {
                    // only down
                    distance += down + ad + da + 1;
                }
            } else {
                if x > prev_x {
                    let right = x - prev_x;

                    // only right
                    distance += right + ar + ra + 1;
                } else if x < prev_x {
                    let left = prev_x - x;

                    // only left
                    distance += left + al + la + 1;
                } else {
                    // nothing <3
                    distance += 1;
                }
            }

            println!("{old_distance} -> {distance} [{}]", distance - old_distance);

            prev_y = y;
            prev_x = x;
        }

        println!("{code:?}: {distance}");

        sum += distance * (code[0].to_digit(10).unwrap() * 100 + code[1].to_digit(10).unwrap() * 10 + code[2].to_digit(10).unwrap()) as usize;
    }

    sum
}

fn parse_input(inp: &str) -> Vec<Vec<char>> {
    inp.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let result = part2(include_str!("example1"), 2);
        assert_eq!(126384, result);
    }

    #[test]
    fn test_part1_real() {
        let result = part2(include_str!("input"), 2);
        assert_eq!(157908, result);
    }

    #[test]
    fn test_part2_real() {
        let result = part2(include_str!("input"), 25);
        println!("Part 2: {}", result);
        assert_eq!(196910339808654, result);
    }
}
