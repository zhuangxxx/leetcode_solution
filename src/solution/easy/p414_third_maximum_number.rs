struct Solution;

impl Solution {
    /// [414. 第三大的数](https://leetcode.cn/problems/third-maximum-number/)
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut set = std::collections::BTreeSet::from_iter(nums.iter());

        **set.iter().collect::<Vec<_>>()[if set.len() < 3 {
            set.len() - 1
        } else {
            set.len() - 3
        }]
        // 0ms/2.3MB
    }

    // pub fn third_max(nums: Vec<i32>) -> i32 {
    //     let mut arr = [None; 3];

    //     for n in nums {
    //         if arr[0].is_none() || n > arr[0].unwrap() {
    //             (arr[2], arr[1], arr[0]) = (arr[1], arr[0], Some(n));
    //         } else if arr[0].unwrap() > n && (arr[1].is_none() || n > arr[1].unwrap()) {
    //             (arr[2], arr[1]) = (arr[1], Some(n));
    //         } else if arr[1].is_some()
    //             && arr[1].unwrap() > n
    //             && (arr[2].is_none() || n > arr[2].unwrap())
    //         {
    //             arr[2] = Some(n);
    //         }
    //     }

    //     if arr[2].is_some() {
    //         arr[2].unwrap()
    //     } else {
    //         arr[0].unwrap()
    //     }
    //     // 0ms/2.1MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::third_max(vec![3, 2, 1]), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::third_max(vec![1, 2]), 2);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::third_max(vec![2, 2, 3, 1]), 1);
    }

    #[test]
    fn trap1() {
        assert_eq!(Solution::third_max(vec![1, 1, 1]), 1);
    }

    #[test]
    fn trap2() {
        assert_eq!(Solution::third_max(vec![1, 1, i32::MIN]), 1);
    }

    #[test]
    fn trap3() {
        assert_eq!(Solution::third_max(vec![1, i32::MIN, i32::MIN]), 1);
    }

    #[test]
    fn trap4() {
        assert_eq!(Solution::third_max(vec![2, 1, i32::MIN]), i32::MIN);
    }

    #[test]
    fn fail1() {
        assert_eq!(Solution::third_max(vec![1, 1, 2]), 2);
    }
}
