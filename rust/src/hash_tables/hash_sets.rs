use std::collections::{HashMap, HashSet};

pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let hash_set1: HashSet<i32> = HashSet::from_iter(nums1.iter().cloned());
    let hash_set2 = HashSet::from_iter(nums2.iter().cloned());
    hash_set1
        .intersection(&hash_set2)
        .into_iter()
        .map(|n| n.clone().clone())
        .collect()
}

pub fn is_happy(n: i32) -> bool {
    let mut happy = true;
    let mut num: u32 = n as u32;
    let mut results = HashSet::new();
    while num != 1 {
        results.insert(num);
        num = num
            .to_string()
            .chars()
            .map(|c| (c.to_digit(10).unwrap()))
            .map(|d| d * d)
            .reduce(|a, b| a + b)
            .unwrap();
        if results.contains(&num) {
            happy = false;
            break;
        }
    }

    happy
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut m: HashMap<&i32, HashSet<i32>> = HashMap::new();
    nums.iter().enumerate().for_each(|(index, num)| {
        if  m.contains_key(&num) {
            let mut updated_index_list = m.get(num).unwrap().to_owned();
            updated_index_list.insert(index as i32);
            m.insert(num, updated_index_list);
        } else {
            let mut index_list = HashSet::new();
            index_list.insert(index as i32);
            m.insert(num, index_list);
        }

    });

    let mut result = Vec::new();
    for (index,num) in nums.iter().enumerate() {
        if m.contains_key(&(target - num)){
            let mut indices = m.get(&(target-num)).unwrap().clone();
            indices.remove(&(index as i32));
            if indices.len() >0 {
                let a: Vec<&i32> = indices.iter().collect();

                result = vec![index as i32, a.first().unwrap().clone().clone()];
                break;
            }
            
        }
        
    }
    result
}

#[cfg(test)]
mod hash_set_tests {
    use super::*;

    #[test]
    fn finds_intersections() {
        let v1 = vec![1, 2, 3];
        let v2 = vec![2, 3, 4];
        let intersection = intersection(v1, v2);
        assert_eq!(intersection, vec![2, 3])
    }

    #[test]
    fn test_happiness() {
        assert!(is_happy(19))
    }
    #[test]
    fn test_happiness2() {
        assert!(is_happy(2))
    }
}
