struct Solution;

impl Solution {
    /// [904. 水果成篮](https://leetcode.cn/problems/fruit-into-baskets/)
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut max = 0;
        let (mut prev, mut curr) = (fruits[0], fruits[0]);
        let (mut b, mut i) = (0, 0);
        for j in 1..fruits.len() {
            if fruits[j] != curr {
                max = std::cmp::max(max, j - i);
                if prev != curr && prev != fruits[j] {
                    i = b;
                }
                b = j;
                prev = curr;
                curr = fruits[j];
            }
        }
        max = std::cmp::max(max, fruits.len() - i);

        max as i32
        // 11ms/2.44MB
    }
}

mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::total_fruit(vec![1, 2, 1]), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::total_fruit(vec![0, 1, 2, 2]), 3);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::total_fruit(vec![1, 2, 3, 2, 2]), 4);
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::total_fruit(vec![3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4]),
            5
        );
    }

    #[test]
    fn fail1() {
        assert_eq!(Solution::total_fruit(vec![3, 3, 3, 1, 4]), 4);
    }
}
