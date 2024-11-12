struct Solution;

impl Solution {
    /// [3258. 统计满足 K 约束的子字符串数量 I](https://leetcode.cn/problems/count-substrings-that-satisfy-k-constraint-i/)
    pub fn count_k_constraint_substrings(s: String, k: i32) -> i32 {
        let mut s = s.as_bytes();
        let mut count = 0;
        let mut c = [0, 0];
        let mut i = 0;
        for j in 0..s.len() {
            let p = (s[j] - b'0') as usize;
            while c[p] >= k && c[p ^ 1] > k {
                c[(s[i] - b'0') as usize] -= 1;
                i += 1;
            }
            count += j - i + 1;
            c[p] += 1;
        }

        count as i32
        // 0ms/2.05MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::count_k_constraint_substrings(String::from("10101"), 1),
            12
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::count_k_constraint_substrings(String::from("1010101"), 2),
            25
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::count_k_constraint_substrings(String::from("11111"), 1),
            15
        );
    }
}
