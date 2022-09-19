fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|n| n != &val);
        nums.len() as i32
}

fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut index_offset = 0;
    for index in 0..nums.len(){
        let next = nums.get(index - index_offset + 1);
        match next {
            Some(n) => {
                if n == &nums[index - index_offset] {
                    nums.remove(index - index_offset);
                    index_offset += 1;
                }
            },
            None => {},
        }
    }

    nums.len() as i32
        
}

#[cfg(test)]
mod array_deletion_tests{
    use super::*;

    #[test]
    fn remove_duplicates_test(){
        let mut input = vec![0,0,1,1,1,1,2,2,3,3,4];
        let k = remove_duplicates(&mut input);
        assert_eq!(k,5);
        assert_eq!(input, vec![0,1,2,3,4])
    }

    #[test]
    fn remove_in_place(){
        let mut nums = vec![3,2,2,3];
        let val = 3;
        let k = remove_element(&mut nums, val);
        assert_eq!(k, 2);
        assert_eq!(nums, vec![2,2])
    }
}