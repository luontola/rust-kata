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
    fn test_string_to_integers() {
        assert_eq!(Vec::<i32>::new(), string_to_integers(""));
        assert_eq!(vec![5], string_to_integers("5"));
        assert_eq!(vec![1, 2, 3], string_to_integers("123"));
    }

    fn string_to_integers(input: &str) -> Vec<i32> {
        input
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as i32))
            .collect()
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
