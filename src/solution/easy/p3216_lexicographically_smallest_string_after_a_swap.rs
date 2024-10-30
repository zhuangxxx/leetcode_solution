struct Solution;

impl Solution {
    /// [3216. 交换后字典序最小的字符串](https://leetcode.cn/problems/lexicographically-smallest-string-after-a-swap/)
    pub fn get_smallest_string(mut s: String) -> String {
        let b = unsafe { s.as_bytes_mut() };

        let mut j = b.len();
        for i in 1..b.len() {
            if b[i - 1] & 1 == b[i] & 1 && b[i - 1] > b[i] {
                j = i;
                break;
            }
        }
        if j < b.len() {
            b.swap(j - 1, j);
        }

        s
        // 0ms/2.03MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::get_smallest_string(String::from("45320")),
            String::from("43520")
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::get_smallest_string(String::from("001")),
            String::from("001")
        );
    }

    #[test]
    fn fail1() {
        assert_eq!(
            Solution::get_smallest_string(String::from("10")),
            String::from("10")
        );
    }
}
