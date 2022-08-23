struct Solution;

impl Solution {
    /// [205. 同构字符串](https://leetcode.cn/problems/isomorphic-strings/)
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let (s, t) = (s.as_bytes(), t.as_bytes());
        let (mut s_prev_i, mut t_prev_i) = (vec![0; 256], vec![0; 256]);

        for i in 0..s.len() {
            if s_prev_i[s[i] as usize] != t_prev_i[t[i] as usize] {
                return false;
            }

            s_prev_i[s[i] as usize] = i + 1;
            t_prev_i[t[i] as usize] = i + 1;
        }

        true
        // 0ms/2.3MB
    }

    // pub fn is_isomorphic(s: String, t: String) -> bool {
    //     let (s, t) = (s.as_bytes(), t.as_bytes());
    //     let (mut s2t, mut t2s) = (
    //         std::collections::HashMap::new(),
    //         std::collections::HashMap::new(),
    //     );

    //     for i in 0..s.len() {
    //         if (s2t.contains_key(&s[i]) && s2t.get(&s[i]).unwrap() != &t[i])
    //             || (t2s.contains_key(&t[i]) && t2s.get(&t[i]).unwrap() != &s[i])
    //         {
    //             return false;
    //         }

    //         s2t.insert(s[i], t[i]);
    //         t2s.insert(t[i], s[i]);
    //     }

    //     true
    //     // 4ms/2.2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_isomorphic(
            "egg".to_string(),
            "add".to_string()
        ));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_isomorphic(
            "foo".to_string(),
            "bar".to_string()
        ));
    }

    #[test]
    fn test3() {
        assert!(Solution::is_isomorphic(
            "paper".to_string(),
            "title".to_string()
        ));
    }
}
