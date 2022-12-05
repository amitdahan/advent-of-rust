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
            assert_eq!(result, 15);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part1(INPUT);
            assert_eq!(result, 13484);
        }
    }

    mod part2 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part2(EXAMPLE);
            assert_eq!(result, 12);
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part2(INPUT);
            assert_eq!(result, 13433);
        }
    }
}

enum GameMove {
    Rock,
    Paper,
    Scissors,
}

enum GameOutcome {
    Win,
    Loss,
    Draw,
}

fn get_outcome(opponent: &GameMove, own: &GameMove) -> GameOutcome {
    match (opponent, own) {
        (GameMove::Rock, GameMove::Rock) => GameOutcome::Draw,
        (GameMove::Paper, GameMove::Rock) => GameOutcome::Loss,
        (GameMove::Scissors, GameMove::Rock) => GameOutcome::Win,
        (GameMove::Rock, GameMove::Paper) => GameOutcome::Win,
        (GameMove::Paper, GameMove::Paper) => GameOutcome::Draw,
        (GameMove::Scissors, GameMove::Paper) => GameOutcome::Loss,
        (GameMove::Rock, GameMove::Scissors) => GameOutcome::Loss,
        (GameMove::Paper, GameMove::Scissors) => GameOutcome::Win,
        (GameMove::Scissors, GameMove::Scissors) => GameOutcome::Draw,
    }
}

pub fn solve_part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let (opponent_move, own_move) = line.split_once(' ').unwrap();
            let opponent_move = match opponent_move {
                "A" => GameMove::Rock,
                "B" => GameMove::Paper,
                "C" => GameMove::Scissors,
                _ => panic!("Invalid opponent move"),
            };
            let own_move = match own_move {
                "X" => GameMove::Rock,
                "Y" => GameMove::Paper,
                "Z" => GameMove::Scissors,
                _ => panic!("Invalid own move"),
            };

            let outcome_value = match get_outcome(&opponent_move, &own_move) {
                GameOutcome::Win => 6,
                GameOutcome::Draw => 3,
                GameOutcome::Loss => 0,
            };
            let own_move_value = match &own_move {
                GameMove::Rock => 1,
                GameMove::Paper => 2,
                GameMove::Scissors => 3,
            };
            outcome_value + own_move_value
        })
        .sum()
}

pub fn solve_part2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let (opponent_move, expected_outcome) = line.split_once(' ').unwrap();
            let opponent_move = match opponent_move {
                "A" => GameMove::Rock,
                "B" => GameMove::Paper,
                "C" => GameMove::Scissors,
                _ => panic!("Invalid opponent move"),
            };
            let expected_outcome = match expected_outcome {
                "X" => GameOutcome::Loss,
                "Y" => GameOutcome::Draw,
                "Z" => GameOutcome::Win,
                _ => panic!("Invalid expected outcome"),
            };

            let own_move = match (&opponent_move, &expected_outcome) {
                (GameMove::Rock, GameOutcome::Loss) => GameMove::Scissors,
                (GameMove::Rock, GameOutcome::Draw) => GameMove::Rock,
                (GameMove::Rock, GameOutcome::Win) => GameMove::Paper,
                (GameMove::Paper, GameOutcome::Loss) => GameMove::Rock,
                (GameMove::Paper, GameOutcome::Draw) => GameMove::Paper,
                (GameMove::Paper, GameOutcome::Win) => GameMove::Scissors,
                (GameMove::Scissors, GameOutcome::Loss) => GameMove::Paper,
                (GameMove::Scissors, GameOutcome::Draw) => GameMove::Scissors,
                (GameMove::Scissors, GameOutcome::Win) => GameMove::Rock,
            };

            let outcome_value = match get_outcome(&opponent_move, &own_move) {
                GameOutcome::Win => 6,
                GameOutcome::Draw => 3,
                GameOutcome::Loss => 0,
            };
            let own_move_value = match &own_move {
                GameMove::Rock => 1,
                GameMove::Paper => 2,
                GameMove::Scissors => 3,
            };
            outcome_value + own_move_value
        })
        .sum()
}
