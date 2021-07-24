/*
 * https://leetcode.com/problems/roman-to-integer/
 *
 * After implementing this solution I looked at the hints which suggested
 * searching backwards may help but I honestly don't see how - either way you
 * need to keep track of the previous match to see if it's a subtraction?
 *
 * Anyway, my solution is pretty simple:
 *
 *  - Iterate through the string, keeping track of the previous character.
 *  - Add the value of each numeral.
 *  - If it turns out it was a subtraction, add the difference between the
 *    subtraction total and what we already added.
 *
 * No doubt there's a fancier method that is more performant, but this one
 * has a runtime of 0ms on the unreliable Leetcode submit test so I guess is
 * ok?
 */

pub struct Solution;

#[derive(Eq, PartialEq)]
enum Prev {
    Unset,
    I,
    X,
    C,
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut rv = 0;
        let mut prev = Prev::Unset;
        for n in s.chars() {
            match n {
                'I' => {
                    rv += 1;
                    prev = Prev::I
                }
                'V' => {
                    rv += if prev == Prev::I { 3 } else { 5 };
                    prev = Prev::Unset
                }
                'X' => {
                    rv += if prev == Prev::I { 8 } else { 10 };
                    prev = Prev::X
                }
                'L' => {
                    rv += if prev == Prev::X { 30 } else { 50 };
                    prev = Prev::Unset
                }
                'C' => {
                    rv += if prev == Prev::X { 80 } else { 100 };
                    prev = Prev::C
                }
                'D' => {
                    rv += if prev == Prev::C { 300 } else { 500 };
                    prev = Prev::Unset
                }
                'M' => {
                    rv += if prev == Prev::C { 800 } else { 1000 };
                    prev = Prev::Unset
                }
                _ => {}
            }
        }
        rv
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
        assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
        assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
