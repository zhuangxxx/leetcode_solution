struct Solution;

impl Solution {
    /// [448. 找到所有数组中消失的数字](https://leetcode.cn/problems/find-all-numbers-disappeared-in-an-array/)
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let (len, mut nums) = (nums.len(), nums);

        for i in 0..len {
            let j = (nums[i] - 1) as usize % len;
            nums[j] += len as i32;
        }

        let mut miss = Vec::new();
        for i in 0..len {
            if nums[i] <= len as i32 {
                miss.push(i as i32 + 1);
            }
        }

        miss
        // 8ms/2.6MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![5, 6]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::find_disappeared_numbers(vec![1, 1]), vec![2]);
    }
}
