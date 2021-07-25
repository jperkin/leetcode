/*
 * https://leetcode.com/problems/search-insert-position/
 *
 * Basically implementing a binary search, but as the target may not exist
 * in the array, if there's no match and the value at the final position is
 * less than the target, we need to return the next position (ignoring whether
 * that is outside the bounds of the array).
 */

pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut min = 0;
        let mut max = nums.len() - 1;
        while min < max {
            let mid = (min + max) / 2;
            match nums[mid].cmp(&target) {
                Ordering::Equal => return mid as i32,
                Ordering::Greater => max = if mid == 0 { 0 } else { mid - 1 },
                Ordering::Less => min = mid + 1,
            }
        }
        if nums[min] < target {
            min as i32 + 1
        } else {
            min as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
        assert_eq!(Solution::search_insert(vec![1], 0), 0);
    }
}
