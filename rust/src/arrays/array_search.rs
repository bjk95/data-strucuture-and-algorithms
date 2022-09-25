pub fn check_if_exist(arr: Vec<i32>) -> bool {
    // Duplicate check
    let mut duplicate_exists = false;
    for outer in 0..arr.len() {
        for inner in 0..arr.len() {
            let i = arr[outer];
            let j = arr[inner];
            if outer != inner && i == 2 * j {
                duplicate_exists = true;
                break;
            }
        }
    }
    duplicate_exists
}

pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
    let mut increasing = true;
    let mut has_increased = false;
    let mut started_descending = false;
    let mut previous = None;
    let mut is_mountain = true;
    for n in arr {
        match previous {
            Some(p) => {
                if n == p {
                    is_mountain = false;
                    break;
                } else if increasing && n > p {
                    previous = Some(n);
                    has_increased = true;
                } else if increasing && n < p {
                    previous = Some(n);
                    increasing = false;
                    started_descending = true;
                    if has_increased == false {
                        break;
                    }
                } else if n < p {
                    previous = Some(n)
                } else {
                    is_mountain = false;
                    break;
                }
            }
            None => previous = Some(n),
        }
    }
    if !increasing && started_descending && is_mountain && has_increased {
        is_mountain = true
    } else {
        is_mountain = false
    }
    is_mountain
}

#[cfg(test)]
mod array_search_tests {
    use super::*;

    #[test]
    fn decreasing_not_mountain() {
        assert_eq!(valid_mountain_array(vec![2, 1]), false)
    }
    #[test]
    fn increasing_not_mountain() {
        assert_eq!(valid_mountain_array(vec![3, 5, 5]), false)
    }
    #[test]
    fn is_mountain() {
        assert_eq!(valid_mountain_array(vec![0, 3, 2, 1]), true)
    }

    #[test]
    fn double_exists() {
        let input = vec![-20, 8, -6, -14, 0, -19, 14, 4];
        assert_eq!(check_if_exist(input), true)
    }

    #[test]
    fn double_does_not_exists() {
        let input = vec![1, 3, 4, 5];
        assert_eq!(check_if_exist(input), false)
    }
}
