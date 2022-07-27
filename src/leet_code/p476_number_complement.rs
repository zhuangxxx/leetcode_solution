struct Solution;

impl Solution {
    /// [476. 数字的补数](https://leetcode.cn/problems/number-complement/)
    pub fn find_complement(num: i32) -> i32 {
        let mut n = num;
        n |= n >> 1;
        n |= n >> 2;
        n |= n >> 4;
        n |= n >> 8;
        n |= n >> 16;

        !num & n
        // 0ms/2.1MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_complement(5), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::find_complement(1), 0);
    }
}
