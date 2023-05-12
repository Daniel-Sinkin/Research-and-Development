use std::fmt::Error;

fn TwoSum(nums: Vec<i32>, target: i32) -> Option<Vec<i32>> {
    for (i, left) in nums.iter().enumerate() {
        for (j, right) in nums.iter().enumerate().skip(i + 1) {
            if left + right == target {
                let iRet = i as i32;
                let jRet = j as i32;
                return Some(vec![iRet, jRet]);
            }
        }
    }

    None
}

fn main() {
    let nums = vec![3, 3];
    let target = 6;

    dbg!(TwoSum(nums, target));
}