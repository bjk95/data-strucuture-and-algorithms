fn duplicate_zeros(arr: &mut Vec<i32>) {
    let length = arr.len();
    let mut offset = 0;
    for i in 0..length {
        if arr.get(i + offset) == Some(&0) {
            arr.insert(offset + i, 0);
            offset += 1;
        }
    }
    arr.resize(length, 0)
}

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let _zeros = nums1.split_off(m as usize);
    let mut a: Vec<i32> = nums1.iter().map(|n| n.clone()).collect();
    a.append(nums2);
    a.sort();
    nums1.clear();
    nums1.append(&mut a)
}
#[cfg(test)]
mod array_insertion_tests {
    use super::*;

    #[test]
    fn duplicate_zeros_no_zeros() {
        let no_zeros: Vec<i32> = vec![1, 2, 3];
        let mut result = no_zeros.clone();
        duplicate_zeros(&mut result);
        assert_eq!(no_zeros, result)
    }

    #[test]
    fn duplicate_zeros_one_zero() {
        let mut one_zero: Vec<i32> = vec![0, 1, 2, 3];
        let result = vec![0, 0, 1, 2];
        duplicate_zeros(&mut one_zero);
        assert_eq!(one_zero, result)
    }

    #[test]
    fn duplicate_zeros_two_zeros() {
        let mut two_zeros: Vec<i32> = vec![0, 1, 0, 2, 3];
        let result = vec![0, 0, 1, 0, 0];
        duplicate_zeros(&mut two_zeros);
        assert_eq!(two_zeros, result)
    }

    #[test]
    fn merge_test() {
        let mut nums1: Vec<i32> = vec![1, 2, 3, 0, 0, 0];
        let mut nums2: Vec<i32> = vec![2, 4, 5];
        let result = vec![1, 2, 2, 3, 4, 5];
        merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, result)
    }
}
