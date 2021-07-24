/*
 * https://leetcode.com/problems/two-sum/
 *
 * The brute force solution iterates twice through the array looking for the
 * two numbers (O(n^2)).  This solution takes advantage of a HashMap to cache
 * the numbers we've already seen, and if we find an entry for the target
 * minus our current number we have a match and can return it.
 *
 * I'm pretty sure there's a clever faster way, but this is mine for now.
 */

pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            let curidx = i as i32;
            match seen.get(&(target - num)) {
                Some(&seenidx) => {
                    return vec![seenidx, curidx];
                }
                None => {
                    seen.insert(num, curidx);
                }
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), [0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), [1, 2]);
        assert_eq!(Solution::two_sum(vec![3, 3], 6), [0, 1]);
    }
}
