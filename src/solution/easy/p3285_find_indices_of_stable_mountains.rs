struct Solution;

impl Solution {
    /// [3285. 找到稳定山的下标](https://leetcode.cn/problems/find-indices-of-stable-mountains/)
    pub fn stable_mountains(height: Vec<i32>, threshold: i32) -> Vec<i32> {
        height
            .windows(2)
            .enumerate()
            .filter_map(|(i, h)| {
                if h[0] > threshold {
                    Some(i as i32 + 1)
                } else {
                    None
                }
            })
            .collect()
        // 0ms/2.35MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::stable_mountains(vec![1, 2, 3, 4, 5], 2),
            vec![3, 4]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::stable_mountains(vec![10, 1, 10, 1, 10], 3),
            vec![1, 3]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::stable_mountains(vec![10, 1, 10, 1, 10], 10),
            Vec::new()
        );
    }
}
