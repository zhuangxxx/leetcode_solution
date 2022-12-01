struct Solution;

impl Solution {
    /// [830. 较大分组的位置](https://leetcode.cn/problems/positions-of-large-groups/)
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        let (mut group, mut start) = (Vec::new(), 0);
        for i in 0..s.len() {
            if s.as_bytes()[start] != s.as_bytes()[i] {
                if i - start >= 3 {
                    group.push(vec![start as i32, i as i32 - 1]);
                }

                start = i;
            } else if i == s.len() - 1 && i - start >= 2 {
                group.push(vec![start as i32, i as i32]);
            }
        }

        group
        // 0ms/1.9MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::large_group_positions("abbxxxxzzy".to_string()),
            vec![vec![3, 6]]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::large_group_positions("abc".to_string()),
            Vec::<Vec<_>>::new()
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::large_group_positions("abcdddeeeeaabbbcd".to_string()),
            vec![vec![3, 5], vec![6, 9], vec![12, 14]]
        );
    }

    #[test]
    fn fail1() {
        assert_eq!(
            Solution::large_group_positions("aaa".to_string()),
            vec![vec![0, 2]]
        );
    }
}
