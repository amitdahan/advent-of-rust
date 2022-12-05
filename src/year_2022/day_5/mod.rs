use std::str::FromStr;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.in");
    const INPUT: &str = include_str!("input.in");

    mod part1 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part1(EXAMPLE);
            assert_eq!(result, "CMZ");
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part1(INPUT);
            assert_eq!(result, "JDTMRWCQJ");
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part2(EXAMPLE);
            assert_eq!(result, "MCD");
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part2(INPUT);
            assert_eq!(result, "VHJDDCWRD");
        }
    }
}

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    let mut stacks = Vec::<Vec<char>>::new();
    let mut lines = input.lines().rev();

    let legend = lines.next().unwrap();
    let legend = legend
        .char_indices()
        .filter(|(_, c)| *c != ' ')
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();

    for _ in &legend {
        stacks.push(vec![]);
    }

    for line in lines {
        for (stack_num, line_idx) in legend.iter().enumerate() {
            let c = line.chars().nth(*line_idx).unwrap();

            if c != ' ' {
                stacks.get_mut(stack_num).unwrap().push(c);
            }
        }
    }
    stacks
}

#[derive(Debug)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (_, input) = input.split_once(' ').unwrap();
        let (count, input) = input.split_once(' ').unwrap();
        let (_, input) = input.split_once(' ').unwrap();
        let (from, input) = input.split_once(' ').unwrap();
        let (_, input) = input.split_once(' ').unwrap();

        let count = count.parse::<usize>().unwrap();
        let from = from.parse::<usize>().unwrap() - 1;
        let to = input.parse::<usize>().unwrap() - 1;

        Ok(Instruction { count, from, to })
    }
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| Instruction::from_str(line).unwrap())
        .collect::<Vec<Instruction>>()
}

pub fn solve_part1(input: &str) -> String {
    let (stacks, instructions) = input.split_once("\n\n").unwrap();

    let mut stacks = parse_stacks(stacks);
    let instructions = parse_instructions(instructions);

    for Instruction { count, from, to } in instructions {
        for _ in 0..count {
            let item = stacks[from].pop().unwrap();
            stacks[to].push(item);
        }
    }

    stacks.iter().map(|s| s.last().unwrap()).collect::<String>()
}

pub fn solve_part2(input: &str) -> String {
    let (stacks, instructions) = input.split_once("\n\n").unwrap();

    let mut stacks = parse_stacks(stacks);
    let instructions = parse_instructions(instructions);

    for Instruction { count, from, to } in instructions {
        let mut temp_stack = Vec::<char>::new();

        for _ in 0..count {
            temp_stack.push(stacks.get_mut(from).unwrap().pop().unwrap());
        }

        for _ in 0..count {
            stacks.get_mut(to).unwrap().push(temp_stack.pop().unwrap());
        }
    }

    stacks.iter().map(|s| s.last().unwrap()).collect::<String>()
}
