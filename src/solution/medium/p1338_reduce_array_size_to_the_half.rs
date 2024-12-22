struct Solution;

impl Solution {
    /// [1338. 数组大小减半](https://leetcode.cn/problems/reduce-array-size-to-the-half/)
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let len = arr.len();
        let mut map = std::collections::HashMap::new();
        for k in arr {
            *map.entry(k).or_insert(0) += 1;
        }
        let mut heap = std::collections::BinaryHeap::new();
        for (k, v) in map {
            heap.push((v, k));
        }
        let (mut size, mut del) = (0, 0);
        while let Some((v, _)) = heap.pop() {
            size += 1;
            del += v;
            if del >= len >> 1 {
                break;
            }
        }

        size
        // 32ms/7.23MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::min_set_size(vec![3, 3, 3, 3, 5, 5, 5, 2, 2, 7]),
            2
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::min_set_size(vec![7, 7, 7, 7, 7, 7]), 1);
    }
}
