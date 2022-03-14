use std::collections::{HashSet, VecDeque};
/// @author: Leon
/// https://leetcode.com/problems/simplify-path/
/// Time Complexity:    O(`len_p`)
/// Space Complexity:   O(`len_p`)
/// Reference:
/// https://leetcode.com/problems/simplify-path/discuss/25686/Java-10-lines-solution-with-stack/168182
/// https://leetcode.com/problems/simplify-path/discuss/25686/Java-10-lines-solution-with-stack
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let _len_p: usize = path.len();
        let skip: HashSet<&str> = HashSet::from(["..", ".", ""]);
        let mut deque: VecDeque<&str> = VecDeque::new();
        for str in path.split("/") {
            if !deque.is_empty() && str.eq("..") {
                deque.pop_back();
            } else if !skip.contains(str) {
                deque.push_back(str);
            }
        }
        if deque.is_empty() {
            return "/".to_owned();
        }
        let mut ans: String = "".to_owned();
        while let Some(top) = deque.pop_front() {
            ans.push('/');
            ans.push_str(top);
        }
        ans
    }
}
