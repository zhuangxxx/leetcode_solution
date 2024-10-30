struct Solution;

impl Solution {
    /// [3211. 生成不含相邻零的二进制字符串](https://leetcode.cn/problems/generate-binary-strings-without-adjacent-zeros/)
    pub fn valid_strings(n: i32) -> Vec<String> {
        let n = n as usize;
        let mask = (1 << n) - 1;
        (0..1 << n)
            .filter_map(|i| {
                let j = mask ^ i;
                if j >> 1 & j == 0 {
                    Some(format!("{i:0n$b}"))
                } else {
                    None
                }
            })
            .collect()
        // 3ms/2.36MB
    }

    // pub fn valid_strings(n: i32) -> Vec<String> {
    //     fn bt(s: &mut [u8], i: usize, n: usize) -> Vec<String> {
    //         if i == n {
    //             return vec![String::from_utf8(s.to_vec()).unwrap_or_default()];
    //         }

    //         let mut r = Vec::new();
    //         if i == 0 || s[i - 1] == b'1' {
    //             s[i] = b'0';
    //             r.append(&mut bt(s, i + 1, n));
    //         }
    //         s[i] = b'1';
    //         r.append(&mut bt(s, i + 1, n));

    //         r
    //     }

    //     bt(&mut vec![b'0'; n as usize], 0, n as usize)
    //     // 7ms/2.71MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::valid_strings(3),
            vec![
                String::from("010"),
                String::from("011"),
                String::from("101"),
                String::from("110"),
                String::from("111")
            ]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::valid_strings(1),
            vec![String::from("0"), String::from("1")]
        );
    }
}
