struct Solution;

impl Solution {
    /// [1436. 旅行终点站](https://leetcode.cn/problems/destination-city/)
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut map = std::collections::HashMap::new();
        for path in paths {
            *map.entry(path[0].to_owned()).or_insert(0) += 1;
            *map.entry(path[1].to_owned()).or_insert(0) += 2;
        }

        for (city, c) in map {
            if c == 2 {
                return city;
            }
        }

        String::new()
        // 2ms/2.07MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::dest_city(vec![
                vec!["London".to_string(), "New York".to_string()],
                vec!["New York".to_string(), "Lima".to_string()],
                vec!["Lima".to_string(), "Sao Paulo".to_string()]
            ]),
            "Sao Paulo".to_string()
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::dest_city(vec![
                vec![String::from("B"), String::from("C")],
                vec![String::from("D"), String::from("B")],
                vec![String::from("C"), String::from("A")]
            ]),
            String::from("A")
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::dest_city(vec![vec![String::from("A"), String::from("Z")]]),
            String::from("Z")
        );
    }
}
