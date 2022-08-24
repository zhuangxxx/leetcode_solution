struct Solution;

impl Solution {
    /// [496. 下一个更大元素 I](https://leetcode.cn/problems/next-greater-element-i/)
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        let mut stack = Vec::new();

        for n2 in nums2.iter().rev() {
            while let Some(top) = stack.last() {
                if n2 < top {
                    break;
                }
                stack.pop();
            }
            map.insert(
                n2,
                if let Some(top) = stack.last() {
                    *top
                } else {
                    -1
                },
            );
            stack.push(*n2);
        }

        let mut next = Vec::new();
        for n1 in nums1 {
            if let Some(&n) = map.get(&n1) {
                next.push(n);
            }
        }

        next
        // 0ms/2.1MB
    }

    // pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    //     let mut next = Vec::new();
    //     for n1 in nums1 {
    //         let mut n = -1;
    //         for j in 0..nums2.len() {
    //             if n != -1 && nums2[j] > n {
    //                 n = nums2[j];
    //                 break;
    //             } else if nums2[j] == n1 {
    //                 n = n1;
    //             }
    //         }
    //         next.push(if n == n1 { -1 } else { n });
    //     }

    //     next
    //     // 4ms/2.2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]),
            vec![-1, 3, -1]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::next_greater_element(vec![2, 4], vec![1, 2, 3, 4]),
            vec![3, -1]
        );
    }
}
