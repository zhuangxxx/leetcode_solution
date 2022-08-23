struct Solution;

impl Solution {
    /// [338. 比特位计数](https://leetcode.cn/problems/counting-bits/)
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut bits_count = vec![0; n as usize + 1];

        let mut i = 1;
        while i <= n as usize {
            for j in 0..=std::cmp::min(i, n as usize - i) {
                bits_count[j + i] = bits_count[j] + 1;
            }
            i <<= 1;
        }

        bits_count
        // 4ms/2.5MB
    }

    // pub fn count_bits(n: i32) -> Vec<i32> {
    //     let mut bits_count = Vec::new();

    //     for i in 0..=n {
    //         let (mut j, mut count) = (i, 0);
    //         while j > 0 {
    //             if j & 1 == 1 {
    //                 count += 1;
    //             }
    //             j >>= 1;
    //         }
    //         bits_count.push(count);
    //     }

    //     bits_count
    //     // 4ms/2.6MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }

    #[test]
    fn trap1() {
        assert_eq!(
            Solution::count_bits(15),
            vec![0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4]
        );
    }
}
