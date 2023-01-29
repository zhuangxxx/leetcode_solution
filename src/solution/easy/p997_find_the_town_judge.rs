struct Solution;

impl Solution {
    /// [997. 找到小镇的法官](https://leetcode.cn/problems/find-the-town-judge/)
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut map = vec![0; n as usize + 1];
        for human in trust {
            map[human[1] as usize] += 1;
            map[human[0] as usize] -= 1;
        }

        for i in 1..=n as usize {
            if map[i] == n - 1 {
                return i as i32;
            }
        }

        -1
        // 20ms/2.6MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_judge(2, vec![vec![1, 2]]), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::find_judge(3, vec![vec![1, 3], vec![2, 3]]), 3);
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::find_judge(3, vec![vec![1, 3], vec![2, 3], vec![3, 1]]),
            -1
        );
    }

    #[test]
    fn fail1() {
        assert_eq!(
            Solution::find_judge(
                4,
                vec![vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![4, 3]]
            ),
            3
        );
    }
}
