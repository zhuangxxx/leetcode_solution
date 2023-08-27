struct Solution;

impl Solution {
    /// [96. 不同的二叉搜索树](https://leetcode.cn/problems/unique-binary-search-trees/)
    pub fn num_trees(n: i32) -> i32 {
        let mut c = 1i64;
        for i in 0..n as i64 {
            c = c * 2 * (2 * i + 1) / (i + 2);
        }

        c as i32
        // 0ms/2.14MB
    }

    // pub fn num_trees(n: i32) -> i32 {
    //     let mut num = vec![0; n as usize + 1];
    //     num[0] = 1;
    //     num[1] = 1;

    //     for i in 2..=n as usize {
    //         for j in 1..=i {
    //             num[i] += num[j - 1] * num[i - j];
    //         }
    //     }

    //     num[n as usize]
    //     // 0ms/1.98MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::num_trees(3), 5);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::num_trees(1), 1);
    }

    #[test]
    fn trap1() {
        assert_eq!(Solution::num_trees(19), 1767263190);
    }
}
