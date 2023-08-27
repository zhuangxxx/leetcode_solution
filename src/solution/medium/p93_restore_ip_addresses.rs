struct Solution;

impl Solution {
    /// [93. 复原 IP 地址](https://leetcode.cn/problems/restore-ip-addresses/)
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        fn dfs(bytes: &[u8], index: usize, ip: &mut [u8; 4], pos: usize) -> Vec<String> {
            if pos >= 4 {
                return if index == bytes.len() {
                    vec![format!("{}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3])]
                } else {
                    Vec::new()
                };
            }
            if index >= bytes.len() {
                return Vec::new();
            }

            let mut n = bytes[index] - b'0';
            ip[pos] = n;

            let mut ips = dfs(bytes, index + 1, ip, pos + 1);
            if n > 0 {
                for i in index + 1..=index + 2 {
                    if i >= bytes.len()
                        || pos >= ip.len()
                        || n > 25
                        || n * 10 > u8::MAX - (bytes[i] - b'0')
                    {
                        break;
                    }

                    n = n * 10 + (bytes[i] - b'0');
                    ip[pos] = n;

                    let mut sub = dfs(bytes, i + 1, ip, pos + 1);
                    if !sub.is_empty() {
                        ips.append(&mut sub);
                    }
                }
            }

            ips
        }

        dfs(s.as_bytes(), 0, &mut [0; 4], 0)
        // 0ms/1.97MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::restore_ip_addresses(String::from("25525511135")),
            vec!["255.255.11.135".to_string(), "255.255.111.35".to_string()]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::restore_ip_addresses(String::from("0000")),
            vec!["0.0.0.0".to_string()]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::restore_ip_addresses(String::from("101023")),
            vec![
                "1.0.10.23".to_string(),
                "1.0.102.3".to_string(),
                "10.1.0.23".to_string(),
                "10.10.2.3".to_string(),
                "101.0.2.3".to_string()
            ]
        );
    }
}
