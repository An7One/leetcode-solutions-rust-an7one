use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/basic-calculator-ii/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`)
/// Note:
/// this is a wrong solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn calculate(s: String) -> i32 {
        const SPACE: char = ' ';
        const SIGN_PLUS: char = '+';
        const SIGN_MINUS: char = '-';
        const SIGN_MULTIPLY: char = '*';
        const SIGN_DIVIDE: char = '/';
        let len_s = s.len();
        let mut stk: VecDeque<i32> = VecDeque::with_capacity(len_s);
        let mut num = 0;
        let mut op = SIGN_PLUS;
        for (idx, ch) in s.chars().into_iter().enumerate() {
            if ch >= '0' && ch <= '9' {
                num = num * 10 + (ch as i32 - '0' as i32);
            } else {
                /*
                 * here is what goes wrong:
                 * for the last number,
                 * it requires calculations based on both blocks, i.e. if{}else{},
                 * thus this is not an relation of `if{}else{}`
                 */
                if ch != SPACE || idx == len_s - 1 {
                    match op {
                        SIGN_PLUS => stk.push_back(num),
                        SIGN_MINUS => stk.push_back(-num),
                        SIGN_MULTIPLY => {
                            if let Some(top) = stk.pop_back() {
                                let product = num * top;
                                stk.push_back(product);
                            }
                        }
                        SIGN_DIVIDE => {
                            if let Some(top) = stk.pop_back() {
                                let quotient = top / num;
                                stk.push_back(quotient);
                            }
                        }
                        _ => {}
                    }
                    op = ch;
                    num = 0;
                }
            }
        }
        println!("{:?}", stk);
        stk.into_iter().sum::<i32>()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_input_1_should_return_expected() {
        let s: String = "3+2*2".to_owned();
        let expected: i32 = 7;
        let actual: i32 = Solution::calculate(s);
        assert_eq!(expected, actual);
    }
}
