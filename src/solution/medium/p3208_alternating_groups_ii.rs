struct Solution;

impl Solution {
    /// [3208. 交替组 II](https://leetcode.cn/problems/alternating-groups-ii/)
    pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
        let mut n = 0;

        let (len, k) = (colors.len(), k as usize);
        let (mut l, mut r) = (0, 1);
        while l < len {
            if r - l + 1 < k {
                if colors[r % len] == colors[(r - 1) % len] {
                    l = r;
                }
            } else {
                if colors[r % len] == colors[(r - 1) % len] {
                    l = r;
                } else {
                    n += 1;
                    l += 1;
                }
            }
            r += 1;
        }

        n
        // 0ms/2.98MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::number_of_alternating_groups(vec![0, 1, 0, 1, 0], 3),
            3
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::number_of_alternating_groups(vec![0, 1, 0, 0, 1, 0, 1], 6),
            2
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::number_of_alternating_groups(vec![1, 1, 0, 1], 4),
            0
        );
    }
}
