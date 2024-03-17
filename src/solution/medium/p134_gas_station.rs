struct Solution;

impl Solution {
    /// [134. 加油站](https://leetcode.cn/problems/gas-station/)
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut start = 0;
        let (mut rem, mut sum) = (0, 0);
        for (i, r) in gas.into_iter().zip(cost).map(|(g, c)| g - c).enumerate() {
            rem += r;
            sum += r;
            if rem < 0 {
                start = i + 1;
                rem = 0;
            }
        }

        if sum < 0 {
            -1
        } else {
            start as i32
        }
        // 9ms/3.04MB
    }
}

mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
            3
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]),
            -1
        );
    }

    #[test]
    fn fail1() {
        assert_eq!(
            Solution::can_complete_circuit(vec![5, 8, 2, 8], vec![6, 5, 6, 6]),
            3
        );
    }
}
