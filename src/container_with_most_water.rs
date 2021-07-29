/*
 * https://leetcode.com/problems/container-with-most-water/
 *
 * This took me a while to figure out the algorithm, and I eventually had to
 * look at one of the hints to realise it was actually a lot simpler than my
 * attempts.  I was trying all sorts, starting from the inside and working out,
 * starting from the highest points, etc.  They passed most tests but not all,
 * and I couldn't figure out what I was missing (but knew they were wrong).
 *
 * The final solution turns out to be trivial.  Start at the ends and work
 * inwards from whichever side is currently smaller.  If the current area is
 * larger than we've seen then save it.  Return the max saved area at the end.
 *
 * Given that the problem was marked as "Medium" I was expecting it to be more
 * difficult, which is probably why I never tried this method, thinking it
 * would be too simple!
 */

pub struct Solution;

fn box_area(lower: i32, upper: i32, distance: usize) -> i32 {
    if upper > lower {
        lower * distance as i32
    } else {
        upper * distance as i32
    }
}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut u = height.len() - 1;
        let mut max = box_area(height[l], height[u], u - l);

        while l < u {
            if height[l] > height[u] {
                u -= 1;
            } else {
                l += 1;
            }
            let tmp = box_area(height[l], height[u], u - l);
            if tmp > max {
                max = tmp;
            }
        }

        max as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode() {
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
        assert_eq!(Solution::max_area(vec![4, 3, 2, 1, 4]), 16);
        assert_eq!(Solution::max_area(vec![1, 2, 1]), 2);
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::max_area(vec![2, 3, 10, 5, 7, 8, 9]), 36);
        assert_eq!(Solution::max_area(vec![2, 3, 4, 5, 18, 17, 6]), 17);
        assert_eq!(Solution::max_area(vec![8, 20, 1, 2, 3, 4, 5, 6]), 42);
        assert_eq!(Solution::max_area(vec![1, 2, 3, 4, 5, 25, 24, 3, 4]), 24);
        assert_eq!(Solution::max_area(vec![1, 8, 100, 2, 100, 4, 8, 3, 7]), 200);
    }
}
