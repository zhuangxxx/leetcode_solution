struct Solution;

impl Solution {
    /// [89. 格雷编码](https://leetcode.cn/problems/gray-code/)
    pub fn gray_code(n: i32) -> Vec<i32> {
        (0..1 << n).map(|i| i >> 1 ^ i).collect()
        // 8ms/2.6MB
    }

    // pub fn gray_code(n: i32) -> Vec<i32> {
    //     let mut code = vec![0];

    //     let mut prev = 1;
    //     for _ in 0..n {
    //         for j in (0..code.len()).rev() {
    //             code.push(prev + code[j]);
    //         }
    //         prev <<= 1;
    //     }

    //     code
    //     // 8ms/2.5MB
    // }

    // pub fn gray_code(n: i32) -> Vec<i32> {
    //     let mut code = vec![0, 1];

    //     let mut prev = 1;
    //     for i in 1..n {
    //         let last = 1 << i;
    //         let curr = prev + last;
    //         code.push(curr);
    //         for j in 1..last as usize / 2 {
    //             code.push(curr + code[j]);
    //         }
    //         for j in (1..last as usize / 2).rev() {
    //             code.push(last + code[j]);
    //         }
    //         code.push(last);
    //         prev = last;
    //     }

    //     code
    //     // 4ms/2.7MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::gray_code(2), vec![0, 1, 3, 2]);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::gray_code(1), vec![0, 1]);
    }
}
