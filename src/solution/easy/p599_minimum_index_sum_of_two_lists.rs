struct Solution;

impl Solution {
    /// [599. 两个列表的最小索引总和](https://leetcode.cn/problems/minimum-index-sum-of-two-lists/)
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut res = Vec::new();

        let mut map = std::collections::HashMap::new();
        for i in 0..list1.len() {
            map.insert(list1[i].to_owned(), i);
        }

        let mut min = 0;
        for i in 0..list2.len() {
            if map.contains_key(&list2[i]) {
                let sum = i + map[&list2[i]];
                if res.is_empty() || sum < min {
                    res = vec![list2[i].to_owned()];
                    min = sum;
                } else if sum == min {
                    res.push(list2[i].to_owned());
                }
            }
        }

        res
        // 16ms/2.4MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_restaurant(
                vec![
                    "Shogun".to_string(),
                    "Tapioca Express".to_string(),
                    "Burger King".to_string(),
                    "KFC".to_string()
                ],
                vec![
                    "Piatti".to_string(),
                    "The Grill at Torrey Pines".to_string(),
                    "Hungry Hunter Steakhouse".to_string(),
                    "Shogun".to_string()
                ]
            ),
            vec!["Shogun".to_string()]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_restaurant(
                vec![
                    "Shogun".to_string(),
                    "Tapioca Express".to_string(),
                    "Burger King".to_string(),
                    "KFC".to_string()
                ],
                vec![
                    "KFC".to_string(),
                    "Shogun".to_string(),
                    "Burger King".to_string()
                ]
            ),
            vec!["Shogun".to_string()]
        );
    }
}
