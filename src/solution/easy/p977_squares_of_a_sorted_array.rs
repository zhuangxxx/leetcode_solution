struct Solution;

impl Solution {
    /// [977. 有序数组的平方](https://leetcode.cn/problems/squares-of-a-sorted-array/)
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut sort = Vec::new();
        let (mut l, mut r) = (0, nums.len() - 1);
        while l <= r {
            let (lp, rp) = (nums[l] * nums[l], nums[r] * nums[r]);
            if lp > rp {
                sort.push(lp);
                l += 1;
            } else if lp < rp {
                sort.push(rp);
                r -= 1;
            } else if l == r {
                sort.push(lp);
                l += 1;
            } else {
                sort.push(lp);
                sort.push(rp);
                l += 1;
                r -= 1;
            }
        }

        sort.reverse();
        sort
        // 12ms/2.2MB
    }

    // pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    //     let mut sort = Vec::new();
    //     let (m, mut l, mut r) = (
    //         'm: loop {
    //             for i in 1..nums.len() {
    //                 if nums[i] > 0 {
    //                     break 'm i - 1;
    //                 }
    //             }

    //             break nums.len() - 1;
    //         },
    //         0,
    //         1,
    //     );
    //     while m >= l && m + r < nums.len() {
    //         let (lp, rp) = (nums[m - l] * nums[m - l], nums[m + r] * nums[m + r]);
    //         if lp < rp {
    //             sort.push(lp);
    //             l += 1;
    //         } else if lp > rp {
    //             sort.push(rp);
    //             r += 1;
    //         } else {
    //             sort.push(lp);
    //             sort.push(rp);
    //             l += 1;
    //             r += 1;
    //         }
    //     }
    //     while l <= m {
    //         sort.push(nums[m - l] * nums[m - l]);
    //         l += 1;
    //     }
    //     while m + r < nums.len() {
    //         sort.push(nums[m + r] * nums[m + r]);
    //         r += 1;
    //     }

    //     sort
    //     // 4ms/2.2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::sorted_squares(vec![-4, -1, 0, 3, 10]),
            vec![0, 1, 9, 16, 100]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::sorted_squares(vec![-7, -3, 2, 3, 11]),
            vec![4, 9, 9, 49, 121]
        );
    }
}
