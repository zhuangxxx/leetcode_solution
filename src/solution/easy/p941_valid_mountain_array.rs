struct Solution;

impl Solution {
    /// [941. 有效的山脉数组](https://leetcode.cn/problems/valid-mountain-array/)
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        if arr.len() < 2 || arr[0] >= arr[1] {
            return false;
        }

        let mut up = true;
        for i in 1..arr.len() {
            if arr[i] == arr[i - 1] {
                return false;
            } else if arr[i] < arr[i - 1] {
                if up {
                    up = false;
                }
            } else if !up {
                return false;
            }
        }

        !up
        // 4ms/2.2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(!Solution::valid_mountain_array(vec![2, 1]));
    }

    #[test]
    fn test2() {
        assert!(!Solution::valid_mountain_array(vec![3, 5, 5]));
    }

    #[test]
    fn test3() {
        assert!(Solution::valid_mountain_array(vec![0, 3, 2, 1]));
    }

    #[test]
    fn fail1() {
        assert!(!Solution::valid_mountain_array(vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9
        ]));
    }
}
