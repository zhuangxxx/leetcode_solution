struct Solution;

impl Solution {
    /// [56. 合并区间](https://leetcode.cn/problems/merge-intervals/)
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable();

        let mut merge = Vec::new();

        let mut interval = intervals[0].clone();
        for i in 1..intervals.len() {
            if interval[1] >= intervals[i][0] {
                interval[1] = std::cmp::max(interval[1], intervals[i][1]);
            } else {
                merge.push(interval);
                interval = intervals[i].clone();
            }
        }
        merge.push(interval);

        merge
        // 12ms/3.2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![4, 5]]),
            vec![vec![1, 5]]
        );
    }

    #[test]
    fn trap1() {
        assert_eq!(Solution::merge(vec![vec![1, 2]]), vec![vec![1, 2]]);
    }

    #[test]
    fn trap2() {
        assert_eq!(
            Solution::merge(vec![vec![1, 2], vec![5, 6], vec![3, 4], vec![2, 3]]),
            vec![vec![1, 4], vec![5, 6]]
        );
    }

    #[test]
    fn trap3() {
        assert_eq!(
            Solution::merge(vec![
                vec![1, 2],
                vec![3, 6],
                vec![5, 6],
                vec![4, 5],
                vec![3, 4]
            ]),
            vec![vec![1, 2], vec![3, 6]]
        );
    }
}
