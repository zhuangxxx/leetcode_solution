struct Solution;

impl Solution {
    /// [49. 字母异位词分组](https://leetcode.cn/problems/group-anagrams/)
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        strs.into_iter()
            .fold(
                std::collections::HashMap::<_, Vec<_>>::new(),
                |mut group, s| {
                    group
                        .entry(s.bytes().fold([0; 26], |mut count, u| {
                            count[(u - b'a') as usize] += 1;
                            count
                        }))
                        .or_default()
                        .push(s);
                    group
                },
            )
            .into_values()
            .collect()
        // 8ms/6.2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut result = Solution::group_anagrams(vec![
            String::from("eat"),
            String::from("tea"),
            String::from("tan"),
            String::from("ate"),
            String::from("nat"),
            String::from("bat"),
        ])
        .into_iter()
        .map(|mut anagram| {
            anagram.sort_unstable();
            anagram
        })
        .collect::<Vec<_>>();
        result.sort_unstable_by_key(|a| a.len());
        assert_eq!(
            result,
            vec![
                vec![String::from("bat")],
                vec![String::from("nat"), String::from("tan")],
                vec![
                    String::from("ate"),
                    String::from("eat"),
                    String::from("tea")
                ]
            ]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::group_anagrams(vec![String::new()]),
            vec![vec![String::new()]]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::group_anagrams(vec![String::from("a")]),
            vec![vec![String::from("a")]]
        );
    }
}
