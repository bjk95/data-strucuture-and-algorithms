fn duplicate_zeros(arr: &mut Vec<i32>) {
    let length = arr.capacity();
    let mut offset = 0;
    for i in 0..length {
        if arr.get(i + offset) == Some(&0){
            arr.insert(i+offset, 0);
            offset += 1;
        }
    }
}

#[cfg(test)]
mod array_insertion_tests {
    use super::*;

    #[test]
    fn duplicate_zeros_no_zeros(){
        let no_zeros: Vec<i32> = vec![1,2,3];
        let mut result = no_zeros.clone();
        duplicate_zeros(&mut result);
        assert_eq!(no_zeros, result)
    }

    #[test]
    fn duplicate_zeros_one_zero(){
        let mut one_zero: Vec<i32> = vec![0,1,2,3];
        let result = vec![0,0,1,2,3];
        duplicate_zeros(&mut one_zero);
        assert_eq!(one_zero, result)
    }

    #[test]
    fn duplicate_zeros_two_zeros(){
        let mut two_zeros: Vec<i32> = vec![0,1,0,2,3];
        let result = vec![0,0,1,0,0,2,3];
        duplicate_zeros(&mut two_zeros);
        assert_eq!(two_zeros, result)
    }
}
