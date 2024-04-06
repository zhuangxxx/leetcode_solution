struct Solution;

impl Solution {
    /// [239. 滑动窗口最大值](https://leetcode.cn/problems/sliding-window-maximum/)
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut queue = std::collections::VecDeque::new();
        for n in &nums[..k] {
            while queue.back().is_some_and(|last| last < n) {
                queue.pop_back();
            }
            queue.push_back(*n);
        }

        let mut result = vec![*queue.front().unwrap()];
        for i in k..nums.len() {
            if queue.front().is_some_and(|first| first == &nums[i - k]) {
                queue.pop_front();
            }
            while queue.back().is_some_and(|last| last < &nums[i]) {
                queue.pop_back();
            }
            queue.push_back(nums[i]);

            result.push(*queue.front().unwrap());
        }

        result
        // 48ms/3.39MB
    }

    // pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    //     let k = k as usize;
    //     let mut heap = std::collections::BinaryHeap::new();
    //     for i in 0..k {
    //         heap.push((nums[i], i));
    //     }

    //     let mut result = Vec::new();
    //     for i in k..nums.len() {
    //         if let Some(&(max, index)) = heap.peek() {
    //             result.push(max);
    //             heap.push((nums[i], i));
    //         }
    //         while let Some(&(_, index)) = heap.peek() {
    //             if index > i - k {
    //                 break;
    //             }
    //             heap.pop();
    //         }
    //     }
    //     if let Some(&(max, _)) = heap.peek() {
    //         result.push(max);
    //     }

    //     result
    //     // 54ms/4.49MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::max_sliding_window(vec![1], 1), vec![1]);
    }

    #[test]
    fn fail1() {
        assert_eq!(
            Solution::max_sliding_window(vec![9, 10, 9, -7, -4, -8, 2, -6], 5),
            vec![10, 10, 9, 2]
        );
    }
}
