struct Solution;

impl Solution {
    /// [697. 数组的度](https://leetcode.cn/problems/degree-of-an-array/)
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::<i32, (i32, usize, usize)>::new();
        for i in 0..nums.len() {
            if let std::collections::hash_map::Entry::Vacant(e) = map.entry(nums[i]) {
                e.insert((1, i, i));
            } else {
                let val = map.entry(nums[i]).or_default();
                val.0 += 1;
                val.2 = i;
            }
        }

        let (mut max, mut len) = (0, 0);
        for (n, (c, b, e)) in map {
            if c == max {
                len = std::cmp::min(len, e - b + 1);
            } else if c > max {
                max = c;
                len = e - b + 1;
            }
        }

        len as i32
        // 8ms/2.6MB
    }

    // pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
    //     let mut map = std::collections::HashMap::new();
    //     for i in 0..nums.len() {
    //         *map.entry(nums[i]).or_insert(0) += 1;
    //     }

    //     let (mut max, mut len) = (Vec::new(), 0);
    //     for (k, v) in map {
    //         if len == v {
    //             max.push(k);
    //         } else if len < v {
    //             max = vec![k];
    //             len = v;
    //         }
    //     }

    //     let mut len = i32::MAX;
    //     for v in max {
    //         let (mut b, mut e) = (-1, -1);
    //         for i in 0..nums.len() {
    //             if nums[i] == v {
    //                 if b == -1 {
    //                     b = i as i32;
    //                 } else {
    //                     e = i as i32;
    //                 }
    //             }
    //         }

    //         len = std::cmp::min(len, e - b + 1);
    //     }

    //     std::cmp::max(len, 1)
    //     // 32ms/2.3MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1]), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1, 4, 2]),
            6
        );
    }

    #[test]
    fn trap1() {
        assert_eq!(Solution::find_shortest_sub_array(vec![1]), 1);
    }

    #[test]
    fn trap2() {
        assert_eq!(Solution::find_shortest_sub_array(vec![1, 2]), 1);
    }
}
