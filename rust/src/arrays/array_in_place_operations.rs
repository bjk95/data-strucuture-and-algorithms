fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
    let mut a = arr.clone();
    for index in 0..a.len() {
        if index == a.len() - 1 {
            a[index] = -1
        } else {
            let mut greatest_to_the_right = 0;
            for right_index in (index + 1)..a.len() {
                if greatest_to_the_right < a[right_index] {
                    greatest_to_the_right = a[right_index];
                }
            }
            a[index] = greatest_to_the_right;
        }
    }
    a
}

fn move_zeroes(nums: &mut Vec<i32>) {
    let mut index_offset = 0;
    for index in 0..nums.len() {
        if nums[index - index_offset] == 0 {
            nums.remove(index - index_offset);
            nums.push(0);
            index_offset += 1;
        }
    }
}

fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
    let mut arr = nums.clone();
    let mut index_offset = 0;
    for index in 0..arr.len() {
        let element = arr[index - index_offset];
        if element % 2 == 0 {
            arr.remove(index - index_offset);
            arr.insert(0, element)
        } else {
            arr.remove(index - index_offset);
            arr.push(element);
            index_offset += 1;
        }
    }
    arr
}

#[cfg(test)]
mod in_place_operation_tests {
    use super::*;

    #[test]
    fn parity_test() {
        let input = vec![1, 3, 5, 0];
        let result = sort_array_by_parity(input);
        assert_eq!(result, vec![0, 1, 3, 5])
    }

    #[test]
    fn move_zeros_test() {
        let mut arr = vec![0, 1, 0, 3, 12];
        move_zeroes(&mut arr);
        assert_eq!(arr, vec![1, 3, 12, 0, 0])
    }
    #[test]
    fn replace_elements_test() {
        let input = vec![17, 18, 5, 4, 6, 1];
        assert_eq!(replace_elements(input), vec![18, 6, 6, 6, 1, -1])
    }
}
