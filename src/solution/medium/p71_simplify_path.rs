struct Solution;

impl Solution {
    /// [71. 简化路径](https://leetcode.cn/problems/simplify-path/)
    pub fn simplify_path(path: String) -> String {
        let (mut stack, mut chars) = (Vec::new(), Vec::new());

        for c in path.chars() {
            if c == '/' {
                let dir: String = chars.iter().collect();
                chars.clear();
                if dir.is_empty() || dir.eq(&".") {
                    continue;
                } else if dir.eq(&"..") {
                    stack.pop();
                } else {
                    stack.push(dir);
                }
            } else {
                chars.push(c);
            }
        }
        if !chars.is_empty() {
            let dir: String = chars.into_iter().collect();
            if dir.is_empty() || dir.eq(&".") {
                // do nothing
            } else if dir.eq(&"..") {
                stack.pop();
            } else {
                stack.push(dir);
            }
        }

        format!("/{}", stack.join("/"))
        // 0ms/2.2MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::simplify_path(String::from("/home/")),
            String::from("/home")
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::simplify_path(String::from("/../")),
            String::from("/")
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::simplify_path(String::from("/home//foo/")),
            String::from("/home/foo")
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::simplify_path(String::from("/a/./b/../../c/")),
            String::from("/c")
        );
    }

    #[test]
    fn trap1() {
        assert_eq!(
            Solution::simplify_path(String::from("/a/./b/../../c")),
            String::from("/c")
        );
    }

    #[test]
    fn trap2() {
        assert_eq!(
            Solution::simplify_path(String::from("/a/./b/../.../c/")),
            String::from("/a/.../c")
        );
    }
}
