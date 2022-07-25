struct Solution;

impl Solution {
    /// [455. 分发饼干](https://leetcode.cn/problems/assign-cookies/)
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let (mut g, mut s) = (g, s);
        g.sort();
        s.sort();

        let (mut i, mut j, mut count) = (0, 0, 0);
        while i < g.len() && j < s.len() {
            if g[i] <= s[j] {
                count += 1;
                i += 1;
                j += 1;
            } else {
                j += 1;
            }
        }

        count
        // 4ms/2.2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_content_children(vec![1, 2, 3], vec![1, 1]),
            1
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_content_children(vec![1, 2], vec![1, 2, 3]),
            2
        );
    }

    #[test]
    fn trap1() {
        assert_eq!(Solution::find_content_children(vec![1], vec![2]), 1);
    }

    #[test]
    fn trap2() {
        assert_eq!(
            Solution::find_content_children(vec![1, 2, 3], vec![4, 4, 4, 4]),
            3
        );
    }

    #[test]
    fn fail1() {
        assert_eq!(
            Solution::find_content_children(vec![1, 2, 4], vec![1, 3]),
            2
        )
    }
}
