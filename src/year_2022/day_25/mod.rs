use itertools::Itertools;
use std::collections::VecDeque;

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.in");
    const INPUT: &str = include_str!("input.in");

    #[test]
    fn it_converts_snafu_to_u64() {
        assert_eq!(snafu_to_u64("0"), 0);
        assert_eq!(snafu_to_u64("1"), 1);
        assert_eq!(snafu_to_u64("2"), 2);
        assert_eq!(snafu_to_u64("1="), 3);
        assert_eq!(snafu_to_u64("1-"), 4);
        assert_eq!(snafu_to_u64("10"), 5);
        assert_eq!(snafu_to_u64("11"), 6);
        assert_eq!(snafu_to_u64("12"), 7);
        assert_eq!(snafu_to_u64("2="), 8);
        assert_eq!(snafu_to_u64("2-"), 9);
        assert_eq!(snafu_to_u64("20"), 10);
        assert_eq!(snafu_to_u64("21"), 11);
        assert_eq!(snafu_to_u64("111"), 31);
        assert_eq!(snafu_to_u64("112"), 32);
        assert_eq!(snafu_to_u64("122"), 37);
        assert_eq!(snafu_to_u64("1-12"), 107);
        assert_eq!(snafu_to_u64("2=0="), 198);
        assert_eq!(snafu_to_u64("2=01"), 201);
        assert_eq!(snafu_to_u64("1=-1="), 353);
        assert_eq!(snafu_to_u64("12111"), 906);
        assert_eq!(snafu_to_u64("20012"), 1257);
        assert_eq!(snafu_to_u64("1=-0-2"), 1747);
    }

    #[test]
    fn it_converts_u64_to_snafu() {
        assert_eq!(u64_to_snafu(0), "0");
        assert_eq!(u64_to_snafu(1), "1");
        assert_eq!(u64_to_snafu(2), "2");
        assert_eq!(u64_to_snafu(3), "1=");
        assert_eq!(u64_to_snafu(4), "1-");
        assert_eq!(u64_to_snafu(5), "10");
        assert_eq!(u64_to_snafu(6), "11");
        assert_eq!(u64_to_snafu(7), "12");
        assert_eq!(u64_to_snafu(8), "2=");
        assert_eq!(u64_to_snafu(9), "2-");
        assert_eq!(u64_to_snafu(10), "20");
        assert_eq!(u64_to_snafu(11), "21");
        assert_eq!(u64_to_snafu(31), "111");
        assert_eq!(u64_to_snafu(32), "112");
        assert_eq!(u64_to_snafu(37), "122");
        assert_eq!(u64_to_snafu(107), "1-12");
        assert_eq!(u64_to_snafu(198), "2=0=");
        assert_eq!(u64_to_snafu(201), "2=01");
        assert_eq!(u64_to_snafu(353), "1=-1=");
        assert_eq!(u64_to_snafu(906), "12111");
        assert_eq!(u64_to_snafu(1257), "20012");
        assert_eq!(u64_to_snafu(1747), "1=-0-2");
    }

    mod part1 {
        use super::*;

        #[test]
        fn it_solves_example() {
            let result = solve_part1(EXAMPLE);
            assert_eq!(result, "2=-1=0");
        }

        #[test]
        fn it_solves_input() {
            let result = solve_part1(INPUT);
            assert_eq!(result, "2=10---0===-1--01-20");
        }
    }
}

fn snafu_to_u64(snafu: &str) -> u64 {
    snafu
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            let value = match c {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => unreachable!(),
            };

            value * 5_i64.pow(i as u32)
        })
        .sum::<i64>() as u64
}

fn u64_to_snafu(mut n: u64) -> String {
    let mut snafu = VecDeque::new();

    if n == 0 {
        return "0".to_string();
    }

    while n > 0 {
        let (snafu_digit, carry_over): (char, i64) = match n % 5 {
            0 => ('0', 0),
            1 => ('1', -1),
            2 => ('2', -2),
            3 => ('=', 2),
            4 => ('-', 1),
            _ => unreachable!(),
        };
        snafu.push_front(snafu_digit);
        n = n.checked_add_signed(carry_over).unwrap();
        n /= 5;
    }

    snafu.iter().join("")
}

pub fn solve_part1(input: &str) -> String {
    u64_to_snafu(input.lines().map(snafu_to_u64).sum())
}
