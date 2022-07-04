struct Solution;

impl Solution {
    /// [169. 多数元素](https://leetcode.cn/problems/majority-element/)
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let (mut winner, mut count) = (nums[0], 1);

        for &num in nums[1..].iter() {
            if winner == num {
                count += 1;
            } else if count == 0 {
                winner = num;
                count += 1;
            } else {
                count -= 1;
            }
        }

        winner
        // 0ms/2.2MB
    }

    // pub fn majority_element(nums: Vec<i32>) -> i32 {
    //     if nums.len() == 1 {
    //         return nums[0];
    //     }

    //     let half = nums.len() as i32 / 2;

    //     let mut map = std::collections::HashMap::new();
    //     for num in nums {
    //         if let Some(&count) = map.get(&num) {
    //             if count + 1 > half {
    //                 return num;
    //             }
    //             map.insert(num, count + 1);
    //         } else {
    //             map.insert(num, 1);
    //         }
    //     }

    //     0
    //     // 4ms/2.5MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }

    #[test]
    fn trap1() {
        assert_eq!(Solution::majority_element(vec![1]), 1);
    }

    #[test]
    fn fail1() {
        assert_eq!(Solution::majority_element(vec![6, 5, 5]), 5);
    }
}
