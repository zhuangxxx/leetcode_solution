struct Solution;

impl Solution {
    /// [347. 前 K 个高频元素](https://leetcode.cn/problems/top-k-frequent-elements/)
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let (mut max, mut map) = (0, std::collections::HashMap::new());
        for n in nums {
            let mut entry = map.entry(n).or_insert(0);
            *entry += 1;
            if max < *entry {
                max = *entry;
            }
        }
        let mut bucket = vec![Vec::new(); max + 1];
        for (n, c) in map {
            bucket[c].push(n);
        }

        bucket
            .into_iter()
            .rev()
            .flatten()
            .take(k as usize)
            .collect()
        // 3ms/2.45MB
    }

    // pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    //     let mut map = std::collections::HashMap::new();
    //     for n in nums {
    //         *map.entry(n).or_insert(0) += 1;
    //     }
    //     let mut heap = std::collections::BinaryHeap::new();
    //     for (n, c) in map {
    //         heap.push((c, n));
    //     }

    //     (0..k).filter_map(|_| heap.pop()).map(|(_, n)| n).collect()
    //     // 3ms/2.43MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2),
            vec![1, 2]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
    }
}
