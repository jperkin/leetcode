/*
 * https://leetcode.com/problems/valid-parentheses/
 *
 * There's probably a fancy way to do a lookup table and simplify some of the
 * code, but my solution is the simple method:
 *
 *  - Create a stack (implemented as a Vector) tracking state.
 *  - If we open a parentheses, push the state type.
 *  - If we close, pop the stack and ensure a matching open type.
 *  - At the end, ensure the stack is empty (parentheses are balanced).
 *
 * Anything else is a failure.
 */

pub struct Solution;

#[derive(Eq, PartialEq)]
enum State {
    Parentheses,
    Braces,
    Brackets,
}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut state: Vec<State> = Vec::new();
        for c in s.chars() {
            match c {
                '(' => state.push(State::Parentheses),
                '{' => state.push(State::Braces),
                '[' => state.push(State::Brackets),
                ')' => {
                    if state.pop() != Some(State::Parentheses) {
                        return false;
                    }
                }
                '}' => {
                    if state.pop() != Some(State::Braces) {
                        return false;
                    }
                }
                ']' => {
                    if state.pop() != Some(State::Brackets) {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        if !state.is_empty() {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leetcode() {
        assert_eq!(Solution::is_valid("()".to_string()), true);
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
        assert_eq!(Solution::is_valid("(]".to_string()), false);
        assert_eq!(Solution::is_valid("([)]".to_string()), false);
        assert_eq!(Solution::is_valid("{[]}".to_string()), true);
    }
}
