struct Solution;

impl Solution {
    /// [1013. 将数组分成和相等的三个部分](https://leetcode.cn/problems/partition-array-into-three-parts-with-equal-sum/)
    pub fn can_three_parts_equal_sum(mut arr: Vec<i32>) -> bool {
        let len = arr.len() - 1;
        for i in 1..arr.len() {
            arr[i] += arr[i - 1];
        }
        if arr[len] % 3 != 0 {
            return false;
        }

        let mut target = arr[len] / 3;
        let mut times = 1;
        for sum in arr.into_iter().take(len) {
            if sum == target * times {
                times += 1;
            }
            if times == 3 {
                break;
            }
        }

        times == 3
        // 8ms/2.6MB
    }

    // pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
    //     let sum: i32 = arr.iter().sum();
    //     if sum % 3 != 0 {
    //         return false;
    //     }

    //     let mut target = sum / 3;
    //     let (mut i, mut s) = (0, 0);
    //     while i < arr.len() - 2 {
    //         s += arr[i];
    //         i += 1;
    //         if s == target {
    //             break;
    //         }
    //     }
    //     if s != target {
    //         return false;
    //     }
    //     target *= 2;
    //     while i < arr.len() - 1 {
    //         s += arr[i];
    //         i += 1;
    //         if s == target {
    //             return true;
    //         }
    //     }

    //     false
    //     // 4ms/2.7MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::can_three_parts_equal_sum(vec![
            0, 2, 1, -6, 6, -7, 9, 1, 2, 0, 1,
        ]));
    }

    #[test]
    fn test2() {
        assert!(!Solution::can_three_parts_equal_sum(vec![
            0, 2, 1, -6, 6, 7, 9, -1, 2, 0, 1,
        ]));
    }

    #[test]
    fn test3() {
        assert!(Solution::can_three_parts_equal_sum(vec![
            3, 3, 6, 5, -2, 2, 5, 1, -9, 4,
        ]));
    }
}
