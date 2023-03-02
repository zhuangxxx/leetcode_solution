struct Solution;

impl Solution {
    /// [22. 括号生成](https://leetcode.cn/problems/generate-parentheses/)
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn backtrack(s: &mut String, open: usize, close: usize, n: usize) -> Vec<String> {
            if s.len() == n * 2 {
                return vec![s.to_owned()];
            }

            let mut p = Vec::new();
            if open < n {
                s.push('(');
                p.append(&mut backtrack(s, open + 1, close, n));
                s.pop();
            }
            if open > close {
                s.push(')');
                p.append(&mut backtrack(s, open, close + 1, n));
                s.pop();
            }

            p
        }

        backtrack(&mut String::new(), 0, 0, n as usize)
        // 0ms/2.2MB
    }

    // pub fn generate_parenthesis(n: i32) -> Vec<String> {
    //     fn gen_pairs(n: i32) -> Vec<String> {
    //         if n == 1 {
    //             return vec![String::from("()")];
    //         }

    //         let mut pairs = std::collections::HashSet::new();
    //         for s in gen_pairs(n - 1) {
    //             pairs.insert(format!("(){}", s));
    //             pairs.insert(format!("(){}", s));

    //             let bytes = s.bytes().collect::<Vec<_>>();
    //             for i in 0..bytes.len() {
    //                 if bytes[i] == b'(' {
    //                     let mut clone = bytes.clone();
    //                     clone.insert(i + 1, b'(');
    //                     clone.insert(i + 2, b')');
    //                     pairs.insert(String::from_utf8_lossy(&clone).into_owned());
    //                 }
    //             }
    //         }

    //         pairs.into_iter().collect()
    //     }

    //     gen_pairs(n)
    //     // 0ms/2.2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut result = Solution::generate_parenthesis(3);
        result.sort_unstable();
        assert_eq!(
            result,
            vec![
                String::from("((()))"),
                String::from("(()())"),
                String::from("(())()"),
                String::from("()(())"),
                String::from("()()()")
            ]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::generate_parenthesis(1), vec![String::from("()")]);
    }
}
