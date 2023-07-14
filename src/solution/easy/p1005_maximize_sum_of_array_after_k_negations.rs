struct Solution;

impl Solution {
    /// [1005. K 次取反后最大化的数组和](https://leetcode.cn/problems/maximize-sum-of-array-after-k-negations/)
    pub fn largest_sum_after_k_negations(nums: Vec<i32>, k: i32) -> i32 {
        let len = nums.len() as i32;
        let (mut count, mut max_neg, mut min_pos) = (0, i32::MIN, i32::MAX);
        let mut has_o = false;
        let mut sum = 0;
        for &n in nums.iter() {
            if n.is_negative() {
                count += 1;
                max_neg = std::cmp::max(n, max_neg);
                sum -= n;
            } else {
                if n == 0 {
                    has_o = true;
                }
                min_pos = std::cmp::min(n, min_pos);
                sum += n;
            }
        }

        sum + match count.cmp(&k) {
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Less => {
                if has_o {
                    0
                } else if k - count & 1 == 1 {
                    if count == 0 {
                        -min_pos
                    } else if count == len {
                        max_neg
                    } else {
                        std::cmp::max(max_neg, -min_pos)
                    }
                } else {
                    0
                }
            }
            std::cmp::Ordering::Greater => {
                let mut heap = std::collections::BinaryHeap::from(
                    nums.into_iter()
                        .filter(|n| n.is_negative())
                        .collect::<Vec<_>>(),
                );
                let mut sub = 0;
                for _ in 0..count - k {
                    if let Some(n) = heap.pop() {
                        sub += n;
                    }
                }

                sub
            }
        } * 2
        // 0ms/2MB
    }

    // pub fn largest_sum_after_k_negations(nums: Vec<i32>, mut k: i32) -> i32 {
    //     let mut bucket = [0; 201];
    //     for n in nums {
    //         bucket[(n + 100) as usize] += 1;
    //     }

    //     let mut sum = 0;
    //     let mut neg = i32::MIN;
    //     for (i, &c) in bucket.iter().enumerate().filter(|(_, c)| c.gt(&&0)) {
    //         let n = i as i32 - 100;
    //         if n.is_negative() {
    //             if k >= c {
    //                 sum += -n * c;
    //                 k -= c;
    //             } else if k > 0 {
    //                 sum += n * (c - k * 2);
    //                 k = 0;
    //             } else {
    //                 sum += n * c;
    //             }

    //             neg = n;
    //         } else {
    //             if k > 0 {
    //                 if n > 0 && k & 1 == 1 {
    //                     sum += std::cmp::max(neg, -n) * 2;
    //                 }
    //                 k = 0;
    //             }

    //             sum += n * c;
    //         }
    //     }

    //     if k & 1 == 1 {
    //         sum += neg * 2;
    //     }

    //     sum
    //     // 0ms/2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::largest_sum_after_k_negations(vec![4, 2, 3], 1), 5);
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::largest_sum_after_k_negations(vec![3, -1, 0, 2], 3),
            6
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::largest_sum_after_k_negations(vec![2, -3, -1, 5, -4], 2),
            13
        );
    }
}
