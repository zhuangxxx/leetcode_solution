struct Solution;

impl Solution {
    /// [771. 宝石与石头](https://leetcode.cn/problems/jewels-and-stones/)
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut map = [0; 55];
        for u in jewels.as_bytes() {
            map[if u > &b'Z' { u - b'a' + 26 } else { u - b'A' } as usize] = 1;
        }

        for u in stones.as_bytes() {
            map[54] += map[if u > &b'Z' { u - b'a' + 26 } else { u - b'A' } as usize];
        }

        map[54]
        // 4ms/2.2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::num_jewels_in_stones(String::from("Aa"), "aAAbbbb".to_string()),
            3
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::num_jewels_in_stones(String::from("z"), "ZZ".to_string()),
            0
        );
    }
}
