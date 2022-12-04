struct Solution;

impl Solution {
    /// [836. 矩形重叠](https://leetcode.cn/problems/rectangle-overlap/)
    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        std::cmp::min(rec1[2], rec2[2]) > std::cmp::max(rec1[0], rec2[0])
            && std::cmp::min(rec1[3], rec2[3]) > std::cmp::max(rec1[1], rec2[1])
        // 0ms/1.9MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::is_rectangle_overlap(
            vec![0, 0, 2, 2],
            vec![1, 1, 3, 3]
        ));
    }

    #[test]
    fn test2() {
        assert!(!Solution::is_rectangle_overlap(
            vec![0, 0, 1, 1],
            vec![1, 0, 2, 1]
        ));
    }

    #[test]
    fn test3() {
        assert!(!Solution::is_rectangle_overlap(
            vec![0, 0, 1, 1],
            vec![2, 2, 3, 3]
        ));
    }

    #[test]
    fn fail1() {
        assert!(Solution::is_rectangle_overlap(
            vec![2, 17, 6, 20],
            vec![3, 8, 6, 20]
        ));
    }
}
