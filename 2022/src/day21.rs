use crate::prelude::*;

use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct MonkeyInfo {
    name: Monkey,
    value: Either<i64, (Op, Monkey, Monkey)>,
}

impl MonkeyInfo {
    fn new() -> Self {
        Self {
            name: Monkey::NULL,
            value: Left(0),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Monkey([u8; 4]);

impl Monkey {
    const NULL: Self = Self([0, 0, 0, 0]);

    const fn idx(&self) -> usize {
        let mut idx = 0;
        let mut i = 0;

        while i < 4 {
            let b = self.0[i];
            debug_assert!(b - b'a' < 26);
            idx *= 26;
            idx += (b - b'a') as usize;
            i += 1;
        }

        idx
    }
}

impl fmt::Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", std::str::from_utf8(&self.0).unwrap())
    }
}

fn resolve_monkey(monkeys: &mut [MonkeyInfo], idx: usize) -> i64 {
    let val = match monkeys[idx].value {
        Left(n) => n,
        Right((op, left, right)) => {
            // 😬😬😬
            let left_idx = monkeys.iter().position(|m| m.name == left).unwrap();
            let right_idx = monkeys.iter().position(|m| m.name == right).unwrap();

            let left = resolve_monkey(monkeys, left_idx);
            let right = resolve_monkey(monkeys, right_idx);

            match op {
                Op::Add => left + right,
                Op::Sub => left - right,
                Op::Mul => left * right,
                Op::Div => left / right,
            }
        }
    };

    monkeys[idx].value = Left(val);
    val
}

// Part1 ========================================================================
#[aoc(day21, part1)]
pub fn part1(input: &str) -> i64 {
    let lines: Vec<_> = input.lines().map(|s| s.as_bytes()).collect();
    let mut monkeys: Vec<MonkeyInfo> = vec![];

    // Parse monkeys
    for line in &lines {
        let mut parts = line.split(|b| *b == b':');
        let name = Monkey(parts.next().unwrap().try_into().unwrap());
        let expr = parts.next().unwrap();
        let value = if expr[1].is_ascii_digit() {
            // ex: " 5"
            Left(fast_parse_u32(&expr[1..]) as i64)
        } else {
            // Everything is really evenly sized
            // ex: " pppw + sjmn"
            let left = Monkey(expr[1..5].try_into().unwrap());
            let right = Monkey(expr[8..12].try_into().unwrap());
            let op = match expr[6] {
                b'+' => Op::Add,
                b'-' => Op::Sub,
                b'*' => Op::Mul,
                b'/' => Op::Div,
                b => unreachable!("Unexpected 'op': {b}"),
            };

            Right((op, left, right))
        };

        let idx = name.idx();

        if monkeys.len() <= idx {
            monkeys.resize_with(idx + 1, MonkeyInfo::new);
        }
        debug_assert_eq!(monkeys[idx].name, Monkey::NULL);
        monkeys[idx] = MonkeyInfo { name, value };
    }

    const ROOT_IDX: usize = Monkey(*b"root").idx();

    // Recursively resolve
    resolve_monkey(&mut monkeys, ROOT_IDX);

    monkeys[ROOT_IDX].value.left().unwrap()
}

// Part2 ========================================================================
// #[aoc(day21, part2)]
// pub fn part2(input: &str) -> i64 {
//     0
// }

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";

    #[rstest]
    #[case::given(152, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }

    // #[rstest]
    // #[case::given(301, EXAMPLE_INPUT)]
    // #[trace]
    // fn check_ex_part_2(
    //     #[notrace]
    //     #[values(part2)]
    //     p: impl FnOnce(&str) -> i64,
    //     #[case] expected: i64,
    //     #[case] input: &str,
    // ) {
    //     let input = input.trim();
    //     assert_eq!(p(input), expected);
    // }
}
