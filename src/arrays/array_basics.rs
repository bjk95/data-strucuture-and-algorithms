fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut max_count = 0;
    for n in nums {
        if n == 1 {
            count += 1;
            if count > max_count {
                max_count = count;
            }
        } else {
            count = 0;
        }
    }
    max_count
}
fn number_of_digits(mut n: i32) -> i32 {
    let mut digits = 0;
    while n >= 1 {
        n /= 10;
        digits += 1;
    }
    digits
}
fn find_numbers(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    for n in nums {
        if number_of_digits(n) % 2 == 0 {
            count += 1;
        }
    }
    count
}

fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut squared: Vec<i32> = nums.iter().map(|n| n * n).collect();
    squared.sort();
    squared
}

#[cfg(test)]
mod array_tests {
    use super::*;

    #[test]
    fn max_ones_test() {
        let nums = vec![1, 0, 1, 1, 0];
        let ones_count = find_max_consecutive_ones(nums);
        assert_eq!(ones_count, 2)
    }

    #[test]
    fn one_digit() {
        assert_eq!(number_of_digits(5), 1)
    }

    #[test]
    fn two_digits() {
        assert_eq!(number_of_digits(50), 2)
    }

    #[test]
    fn three_digits() {
        assert_eq!(number_of_digits(555), 3)
    }

    #[test]
    fn find_1_even() {
        let nums = vec![1, 20, 300];
        assert_eq!(find_numbers(nums), 1)
    }

    #[test]
    fn find_3_evens() {
        let nums = vec![1, 20, 300, 666, 4000, 600000];
        assert_eq!(find_numbers(nums), 3)
    }

    #[test]
    fn test_sqaures() {
        let nums = vec![1, 2, 3, 4];
        let expected_result = vec![1, 4, 9, 16];
        assert_eq!(sorted_squares(nums), expected_result)
    }
}
