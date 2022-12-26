struct Solution;

impl Solution {
    /// [929. 独特的电子邮件地址](https://leetcode.cn/problems/unique-email-addresses/)
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        emails
            .into_iter()
            .map(|email| {
                if let Some(p) = email.find('@') {
                    format!(
                        "{}{}",
                        &email[..email.find('+').unwrap_or(p)].replace(".", ""),
                        &email[p..]
                    )
                } else {
                    String::new()
                }
            })
            .collect::<std::collections::HashSet<_>>()
            .len() as i32
        // 4ms/2.1MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::num_unique_emails(vec![
                "test.email+alex@leetcode.com".to_string(),
                "test.e.mail+bob.cathy@leetcode.com".to_string(),
                "testemail+david@lee.tcode.com".to_string()
            ]),
            2
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::num_unique_emails(vec![
                "a@leetcode.com".to_string(),
                "b@leetcode.com".to_string(),
                "c@lee.tcode.com".to_string()
            ]),
            3
        );
    }
}
