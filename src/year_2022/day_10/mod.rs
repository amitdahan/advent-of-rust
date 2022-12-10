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
            assert_eq!(result, 13140);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part1(INPUT);
            assert_eq!(result, 11220);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part2(EXAMPLE);

            assert_eq!(
                result,
                "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
            );
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part2(INPUT);
            assert_eq!(
                result,
                "###..####.###...##....##.####.#....#..#.
#..#....#.#..#.#..#....#.#....#....#.#..
###....#..#..#.#..#....#.###..#....##...
#..#..#...###..####....#.#....#....#.#..
#..#.#....#....#..#.#..#.#....#....#.#..
###..####.#....#..#..##..####.####.#..#."
            );
        }
    }
}

#[derive(Debug)]
enum Command {
    AddX(i32),
    Noop,
}

impl Command {
    fn cycles(&self) -> usize {
        match self {
            Command::AddX(_) => 2,
            Command::Noop => 1,
        }
    }
}

fn parse_commands(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|line| match line.split_once(' ') {
            Some(("addx", n)) => Command::AddX(n.parse().unwrap()),
            _ => Command::Noop,
        })
        .collect()
}

const INTERESTING_CYCLES: [usize; 6] = [20, 60, 100, 140, 180, 220];

pub fn solve_part1(input: &str) -> i32 {
    let commands = parse_commands(input);
    let mut cycle = 0;
    let mut x: i32 = 1;
    let mut sum: i32 = 0;

    while cycle < INTERESTING_CYCLES[INTERESTING_CYCLES.len() - 1] {
        for command in &commands {
            for _ in 0..command.cycles() {
                cycle += 1;

                if INTERESTING_CYCLES.contains(&cycle) {
                    sum += cycle as i32 * x;
                }
            }

            match command {
                Command::AddX(n) => x += n,
                Command::Noop => {}
            }
        }
    }

    sum
}

const CRT_WIDTH: usize = 40;
const CRT_HEIGHT: usize = 6;

pub fn solve_part2(input: &str) -> String {
    let commands = parse_commands(input);
    let mut cycle = 0;
    let mut x: i32 = 1;
    let mut crt = [[false; CRT_WIDTH]; CRT_HEIGHT];

    for command in &commands {
        for _ in 0..command.cycles() {
            let row = cycle / CRT_WIDTH;
            let col = cycle % CRT_WIDTH;
            crt[row][col] = col as i32 - x >= -1 && col as i32 - x <= 1;
            cycle += 1;
        }

        match command {
            Command::AddX(n) => x += n,
            Command::Noop => {}
        };
    }

    crt.iter()
        .map(|row| {
            row.iter()
                .map(|&b| if b { '#' } else { '.' })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n")
}
