use std::collections::{HashMap, HashSet, VecDeque};

/// @author: Leon
/// https://leetcode.com/problems/accounts-merge/
/// Time Complexity:    O()
/// Space Complexity:   O()
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut ans: Vec<Vec<String>> = Vec::new();
        let (graph, email_to_name): (HashMap<String, HashSet<String>>, HashMap<String, String>) =
            Self::build_graph(&accounts);
        let mut seen: HashSet<String> = HashSet::new();
        for (email, _) in graph.iter() {
            if !seen.insert(email.to_owned()) {
                continue;
            }
            let mut res = Self::bfs(email, &mut seen, &graph);
            res.sort();
            res.insert(0, email_to_name.get(&res[0].to_owned()).unwrap().to_owned());
            ans.push(res);
        }
        return ans;
    }
    fn bfs(
        start: &str,
        seen: &mut HashSet<String>,
        graph: &HashMap<String, HashSet<String>>,
    ) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        let mut queue: VecDeque<String> = {
            let mut queue: VecDeque<String> = VecDeque::new();
            queue.push_back(start.to_owned());
            queue
        };
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            for _ in 0..len_q {
                if let Some(cur) = queue.pop_front() {
                    res.push(cur.to_owned());
                    if let Some(nxts) = graph.get(&cur) {
                        for nxt in nxts {
                            if seen.insert(nxt.to_owned()) {
                                queue.push_back(nxt.to_owned());
                            }
                        }
                    }
                }
            }
        }
        return res;
    }
    fn build_graph(
        accounts: &Vec<Vec<String>>,
    ) -> (HashMap<String, HashSet<String>>, HashMap<String, String>) {
        let _len_as: usize = accounts.len();
        let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
        let mut email_to_name: HashMap<String, String> = HashMap::new();
        for account in accounts {
            let len = account.len();
            let name = &account[0];
            for idx in 1..len {
                graph.entry(account[idx].to_owned()).or_default();
                email_to_name.insert(account[idx].to_owned(), name.to_owned());
                if idx > 1 {
                    let u = &account[idx - 1];
                    let v = &account[idx];
                    graph.entry(u.to_owned()).or_default().insert(v.to_owned());
                    graph.entry(v.to_owned()).or_default().insert(u.to_owned());
                }
            }
        }
        return (graph, email_to_name);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let accounts: Vec<Vec<String>> = vec![
            vec![
                "John".to_owned(),
                "johnsmith@mail.com".to_owned(),
                "john_newyork@mail.com".to_owned(),
            ],
            vec![
                "John".to_owned(),
                "johnsmith@mail.com".to_owned(),
                "john00@mail.com".to_owned(),
            ],
            vec!["Mary".to_owned(), "mary@mail.com".to_owned()],
            vec!["John".to_owned(), "johnnybravo@mail.com".to_owned()],
        ];
        let expected: Vec<Vec<String>> = vec![
            vec![
                "John".to_owned(),
                "john00@mail.com".to_owned(),
                "john_newyork@mail.com".to_owned(),
                "johnsmith@mail.com".to_owned(),
            ],
            vec!["Mary".to_owned(), "mary@mail.com".to_owned()],
            vec!["John".to_owned(), "johnnybravo@mail.com".to_owned()],
        ];
        let actual: Vec<Vec<String>> = Solution::accounts_merge(accounts);
        assert_eq!(expected, actual);
    }
}
