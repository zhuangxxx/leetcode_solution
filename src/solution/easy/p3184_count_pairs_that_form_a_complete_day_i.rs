struct Solution;

impl Solution {
    /// [3184. 构成整天的下标对数目 I](https://leetcode.cn/problems/count-pairs-that-form-a-complete-day-i/)
    pub fn count_complete_day_pairs(hours: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        for hour in hours {
            map.entry(match hour % 24 {
                0 => 0,
                rem => 24 - rem,
            })
            .and_modify(|x| *x += 1)
            .or_insert(1);
        }

        let mut s = 0;

        let mut v = std::collections::HashSet::new();
        for k in map.keys() {
            if v.contains(k) {
                continue;
            }

            s += match k {
                0 | 12 => {
                    if map[k] > 1 {
                        map[k] * (map[k] - 1) >> 1
                    } else {
                        0
                    }
                }
                _ => {
                    v.insert(24 - k);
                    if let Some(n) = map.get(&(24 - k)) {
                        n * map[k]
                    } else {
                        0
                    }
                }
            };
        }

        s
        // 0ms/2.03MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::count_complete_day_pairs(vec![12, 12, 30, 24, 24]),
            2
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::count_complete_day_pairs(vec![72, 48, 24, 3]), 3);
    }
}
