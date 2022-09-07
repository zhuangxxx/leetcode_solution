struct Solution;

impl Solution {
    /// [645. 错误的集合](https://leetcode.cn/problems/set-mismatch/)
    pub fn find_error_nums(mut nums: Vec<i32>) -> Vec<i32> {
        let total = ((1 + nums.len()) * nums.len()) as i32 / 2;
        let (mut map, mut sum, mut set) = (vec![0; nums.len()], 0, 0);
        for n in nums {
            sum += n;
            if map[n as usize - 1] == 0 {
                set += n;
            }
            map[n as usize - 1] = 1;
        }

        vec![sum - set, total - set]
        // 0ms/2.2MB
    }

    // pub fn find_error_nums(mut nums: Vec<i32>) -> Vec<i32> {
    //     let mut xor_sum = 0;
    //     for i in 1..=nums.len() {
    //         xor_sum ^= i as i32;
    //         xor_sum ^= nums[i - 1];
    //     }

    //     let mut err = vec![0, 0];
    //     let low_bit = xor_sum & (-xor_sum);
    //     for i in 0..nums.len() {
    //         if nums[i] & low_bit == 0 {
    //             err[0] ^= nums[i];
    //         } else {
    //             err[1] ^= nums[i];
    //         }
    //     }
    //     for i in 1..=nums.len() as i32 {
    //         if i & low_bit == 0 {
    //             err[0] ^= i;
    //         } else {
    //             err[1] ^= i;
    //         }
    //     }
    //     for n in nums {
    //         if n == err[0] {
    //             return err;
    //         }
    //     }

    //     err.swap(0, 1);

    //     err
    //     // 4ms/2.1MB
    // }

    // pub fn find_error_nums(mut nums: Vec<i32>) -> Vec<i32> {
    //     let (mut err, mut map) = (vec![0, 0], vec![0; nums.len()]);
    //     for n in nums {
    //         map[n as usize - 1] += 1;
    //     }

    //     for i in 0..map.len() {
    //         if map[i] == 2 {
    //             err[0] = i as i32 + 1;
    //         } else if map[i] == 0 {
    //             err[1] = i as i32 + 1;
    //         }
    //     }

    //     err
    //     // 0ms/2.1MB
    // }

    // pub fn find_error_nums(mut nums: Vec<i32>) -> Vec<i32> {
    //     nums.sort();

    //     let (mut err, mut prev) = (vec![0, 0], 0);
    //     for i in 0..nums.len() {
    //         let curr = nums[i];
    //         if curr == prev {
    //             err[0] = curr;
    //         } else if curr - prev > 1 {
    //             err[1] = prev + 1;
    //         }
    //         prev = curr;
    //     }

    //     if nums[nums.len() - 1] != nums.len() as i32 {
    //         err[1] = nums.len() as i32;
    //     }

    //     err
    //     // 8ms/2.1MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_error_nums(vec![1, 2, 2, 4]), vec![2, 3]);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::find_error_nums(vec![1, 1]), vec![1, 2]);
    }

    #[test]
    fn trap1() {
        assert_eq!(
            Solution::find_error_nums(vec![1, 3, 5, 7, 2, 10, 8, 6, 4, 2]),
            vec![2, 9]
        );
    }

    #[test]
    fn fail1() {
        assert_eq!(
            Solution::find_error_nums(vec![
                16, 27, 24, 43, 11, 2, 26, 13, 21, 41, 40, 48, 37, 39, 47, 23, 8, 12, 22, 42, 7,
                38, 29, 1, 20, 3, 4, 44, 18, 14, 46, 33, 28, 30, 32, 36, 15, 9, 17, 45, 19, 31, 25,
                5, 4, 34, 35, 10
            ]),
            vec![4, 6]
        );
    }

    #[test]
    fn fail2() {
        assert_eq!(Solution::find_error_nums(vec![2, 2]), vec![2, 1]);
    }
}
