fn main() {
    println!("Hello, world!");
}

// https://adventofcode.com/2017/day/1
#[cfg(test)]
mod aoc2017_day1_tests {
    #[test]
    fn acceptance_tests() {
        assert_eq!(3, evaluate("1122"));
        assert_eq!(4, evaluate("1111"));
        assert_eq!(0, evaluate("1234"));
        assert_eq!(9, evaluate("91212129"));
    }

    #[test]
    fn test_empty() {
        assert_eq!(0, evaluate(""));
    }

    #[test]
    fn test_no_matching_pairs() {
        assert_eq!(0, evaluate("1234"));
    }

    #[test]
    fn test_one_matching_pair_without_rollover() {
        assert_eq!(1, evaluate("112"));
        assert_eq!(1 + 2, evaluate("1122"));
    }

    #[test]
    fn test_multiple_matching_pairs_without_rollover() {
        assert_eq!(1 + 1, evaluate("1112"));
        assert_eq!(1 + 1 + 2 + 2, evaluate("111222"));
    }

    #[test]
    fn test_rollover() {
        assert_eq!(1 + 1 + 1, evaluate("111"));
        assert_eq!(2 + 2, evaluate("22"));
        assert_eq!(7, evaluate("7"));
    }

    fn evaluate(string: &str) -> u32 {
        let mut numbers = string_to_integers(string);
        if numbers.is_empty() {
            return 0;
        }
        numbers.push(numbers[0]);
        let mut sum = 0u32;
        for pair in numbers.windows(2) {
            sum += evaluate_pair(pair[0], pair[1])
        }
        sum
    }

    #[test]
    fn test_string_to_integers() {
        assert_eq!(Vec::<u32>::new(), string_to_integers(""));
        assert_eq!(vec![5], string_to_integers("5"));
        assert_eq!(vec![1, 2, 3], string_to_integers("123"));
    }

    fn string_to_integers(input: &str) -> Vec<u32> {
        input
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect()
    }

    #[test]
    fn test_evaluate_pair() {
        assert_eq!(1, evaluate_pair(1, 1));
        assert_eq!(0, evaluate_pair(1, 2));
        assert_eq!(6, evaluate_pair(6, 6));
    }

    fn evaluate_pair(a: u32, b: u32) -> u32 {
        if a == b {
            a
        } else {
            0
        }
    }
}
