struct Solution;

impl Solution {
    /// [2545. 根据第 K 场考试的分数排序](https://leetcode.cn/problems/sort-the-students-by-their-kth-score/)
    pub fn sort_the_students(mut score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        score.sort_unstable_by(|a, b| b[k as usize].cmp(&a[k as usize]));

        score
        // 0ms/3.02MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::sort_the_students(
                vec![vec![10, 6, 9, 1], vec![7, 5, 11, 2], vec![4, 8, 3, 15]],
                2
            ),
            vec![vec![7, 5, 11, 2], vec![10, 6, 9, 1], vec![4, 8, 3, 15]]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::sort_the_students(vec![vec![3, 4], vec![5, 6]], 0),
            vec![vec![5, 6], vec![3, 4]]
        );
    }
}
