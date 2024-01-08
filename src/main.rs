fn main() {
    println!("Hello, world!");
}

// https://adventofcode.com/2017/day/1
#[cfg(test)]
mod aoc2017_day1_tests {
    #[test]
    fn test_no_matching_pairs() {
        assert_eq!(0, evaluate("1234"));
    }

    fn evaluate(string: &str) -> i32 {
        0
    }

    #[test]
    fn test_evaluate_pair() {
        assert_eq!(1, evaluate_pair(1, 1));
        assert_eq!(0, evaluate_pair(1, 2));
        assert_eq!(6, evaluate_pair(6, 6));
    }

    fn evaluate_pair(a: i32, b: i32) -> i32 {
        if a == b {
            a
        } else {
            0
        }
    }
}
