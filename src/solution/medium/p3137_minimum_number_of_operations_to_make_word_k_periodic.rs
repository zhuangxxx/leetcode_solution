struct Solution;

impl Solution {
    /// [3137. K 周期字符串需要的最少操作次数](https://leetcode.cn/problems/minimum-number-of-operations-to-make-word-k-periodic/)
    pub fn minimum_operations_to_make_k_periodic(word: String, k: i32) -> i32 {
        let bytes = word.as_bytes();

        let (mut sum, mut max) = (0, 0);
        let mut map = std::collections::HashMap::new();
        for i in (0..bytes.len()).step_by(k as usize) {
            let v = map.entry(&bytes[i..i + k as usize]).or_insert(0);
            *v += 1;

            sum += 1;
            max = std::cmp::max(max, *v);
        }

        sum - max
        // 16ms/3.34MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::minimum_operations_to_make_k_periodic(String::from("leetcodeleet"), 4),
            1
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::minimum_operations_to_make_k_periodic(String::from("leetcoleet"), 2),
            3
        );
    }
}
