struct Solution;

impl Solution {
    /// [15. 三数之和](https://leetcode.cn/problems/3sum/)
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        if nums[nums.len() - 1] < 0 {
            return Vec::new();
        }

        let mut vec = Vec::new();
        for i in 0..nums.len() - 2 {
            if nums[i] > 0 {
                break;
            }
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let (mut l, mut r) = (i + 1, nums.len() - 1);
            while l < r {
                let s = nums[i] + nums[l] + nums[r];
                if s < 0 {
                    l += 1;
                } else if s > 0 {
                    r -= 1;
                } else {
                    vec.push(vec![nums[i], nums[l], nums[r]]);
                    while l < r && nums[l] == nums[l + 1] {
                        l += 1;
                    }
                    while l < r && nums[r] == nums[r - 1] {
                        r -= 1;
                    }
                    l += 1;
                    r -= 1;
                }
            }
        }

        vec
        // 36ms/3.9MB
    }

    // pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    //     nums.sort_unstable();
    //     let (m, x) = (nums[0], nums[nums.len() - 1]);
    //     if m > 0 || x < 0 {
    //         return Vec::new();
    //     }

    //     let mut o = 0;
    //     for i in 0..nums.len() {
    //         if nums[i].ge(&0) {
    //             o = i;
    //             break;
    //         }
    //     }
    //     if nums[o].eq(&0) && ((o.eq(&0) && nums[o + 2].ne(&0)) || o.eq(&(nums.len() - 1))) {
    //         return Vec::new();
    //     }

    //     let mut set = std::collections::HashSet::new();
    //     if o.le(&(nums.len() - 3)) && nums[o + 2].eq(&0) {
    //         set.insert(vec![0, 0, 0]);
    //     }
    //     for i in 0..nums.len() - 1 {
    //         for j in i + 1..nums.len() {
    //             if nums[i].eq(&0) && nums[j].eq(&0) {
    //                 continue;
    //             }

    //             let t = 0 - nums[i] - nums[j];
    //             if t.lt(&m) || t.gt(&x) {
    //                 continue;
    //             }

    //             let sum = if t.lt(&nums[i]) {
    //                 vec![t, nums[i], nums[j]]
    //             } else if t.gt(&nums[j]) {
    //                 vec![nums[i], nums[j], t]
    //             } else {
    //                 vec![nums[i], t, nums[j]]
    //             };
    //             if set.contains(&sum) {
    //                 continue;
    //             }

    //             let (mut l, mut r) = if t.le(&0) {
    //                 (0, o)
    //             } else {
    //                 (o, nums.len() - 1)
    //             };
    //             while l < r {
    //                 let mut c = l + (r - l) / 2;
    //                 if t.lt(&nums[c]) {
    //                     r = c;
    //                 } else if t.gt(&nums[c]) {
    //                     l = c + 1;
    //                 } else {
    //                     if (c.ne(&i) && c.ne(&j))
    //                         || (c.eq(&i)
    //                             && (t.eq(&nums[i + 1]) || (i.gt(&0) && t.eq(&nums[i - 1]))))
    //                         || (c.eq(&j)
    //                             && (t.eq(&nums[j - 1])
    //                                 || (j.lt(&(nums.len() - 1)) && t.eq(&nums[j + 1]))))
    //                     {
    //                         set.insert(sum);
    //                     }
    //                     break;
    //                 }
    //             }
    //         }
    //     }

    //     set.into_iter().collect()
    //     // 484ms/4MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut result = Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]);
        result.sort_unstable();
        assert_eq!(result, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::three_sum(vec![0, 1, 1]), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
    }

    #[test]
    fn fail1() {
        assert_eq!(Solution::three_sum(vec![-1, 0, 1, 0]), vec![vec![-1, 0, 1]]);
    }

    #[test]
    fn fail2() {
        let mut result =
            Solution::three_sum(vec![-4, -2, 1, -5, -4, -4, 4, -2, 0, 4, 0, -2, 3, 1, -5, 0]);
        result.sort_unstable();
        assert_eq!(
            result,
            vec![
                vec![-5, 1, 4],
                vec![-4, 0, 4],
                vec![-4, 1, 3],
                vec![-2, -2, 4],
                vec![-2, 1, 1],
                vec![0, 0, 0]
            ]
        );
    }
}
