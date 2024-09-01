struct Solution;

impl Solution {
    /// [1450. 在既定时间做作业的学生人数](https://leetcode.cn/problems/number-of-students-doing-homework-at-a-given-time/)
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        start_time.into_iter().zip(end_time).fold(0, |acc, (s, e)| {
            if s <= query_time && query_time <= e {
                acc + 1
            } else {
                acc
            }
        })
        // 2ms/2.18MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::busy_student(vec![1, 2, 3], vec![3, 2, 7], 4), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::busy_student(vec![4], vec![4], 4), 1);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::busy_student(vec![4], vec![4], 5), 0);
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::busy_student(vec![1, 1, 1, 1], vec![1, 3, 2, 4], 7),
            0
        );
    }

    #[test]
    fn test5() {
        assert_eq!(
            Solution::busy_student(
                vec![9, 8, 7, 6, 5, 4, 3, 2, 1],
                vec![10, 10, 10, 10, 10, 10, 10, 10, 10],
                5
            ),
            5
        );
    }
}
