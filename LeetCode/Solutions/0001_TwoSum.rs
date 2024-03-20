use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Option<(usize, usize)> {
    // Iterate through loop once = O(n)
    // Hash lookup assumed to be O(1) => Time = O(n)
    // Storing hashmap => Space = O(n)
    let mut hm: HashMap<i32, usize> = HashMap::new();

    let mut curr: i32;
    for (i, n) in nums.iter().enumerate() {
        curr = target - n;

        if let Some(ind) = hm.get(&curr) {
            println!("sum*(nums[{}] = {n}, nums[{}] = {curr}) = {target}", i, ind);
            return Some((*ind, i));
        }

        hm.insert(*n, i);
    }

    return None;
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;

    dbg!(two_sum(nums, target));
}

#[cfg(test)]
mod tests {
    use crate::two_sum;
    #[test]
    fn testCase1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;

        assert_eq!(two_sum(nums, target), Some((0, 1)));
    }

    #[test]
    fn testCase2() {
        let nums = vec![3, 2, 4];
        let target = 6;

        assert_eq!(two_sum(nums, target), Some((1, 2)));
    }

    #[test]
    fn testCase3() {
        let nums = vec![3, 3];
        let target = 6;

        assert_eq!(two_sum(nums, target), Some((0, 1)));
    }

    #[test]
    fn testErrorNotFound() {
        let nums = vec![1, 2, 3, 4, 5, 6, 7];
        let target = -1;


        assert_eq!(two_sum(nums, target), None);
    }

    #[test]
    fn testErrorEmptyVector() {
        let nums =  vec![];
        let target = 0;

        assert_eq!(two_sum(nums, target), None);
    }

    #[test]
    fn testErrorOneElementVector() {
        let nums =  vec![1];
        let target = 0;

        assert_eq!(two_sum(nums, target), None);
    }
}