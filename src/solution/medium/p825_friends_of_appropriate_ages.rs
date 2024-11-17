struct Solution;

impl Solution {
    /// [825. 适龄的朋友](https://leetcode.cn/problems/friends-of-appropriate-ages/)
    pub fn num_friend_requests(ages: Vec<i32>) -> i32 {
        let mut count = [0; 121];
        for age in ages {
            count[age as usize] += 1;
        }
        let mut prefix = [0; 121];
        for i in 1..=120 {
            prefix[i] = prefix[i - 1] + count[i];
        }
        let mut num = 0;
        for i in (15..=120).rev() {
            if count[i] == 0 {
                continue;
            }

            num += count[i] * (prefix[i] - prefix[(i >> 1) + 7] - 1);
        }

        num
        // 0ms/2.24MB
    }

    // pub fn num_friend_requests(ages: Vec<i32>) -> i32 {
    //     let mut count = [0; 121];
    //     for age in ages {
    //         count[age as usize] += 1;
    //     }
    //     let mut num = 0;
    //     for i in (1..=120).rev() {
    //         if count[i] == 0 {
    //             continue;
    //         }
    //         let mut ni = 0;
    //         for j in (i >> 1) + 8..=i {
    //             if count[j] == 0 {
    //                 continue;
    //             }
    //             ni += if i != j { count[j] } else { count[j] - 1 };
    //         }

    //         num += count[i] * ni;
    //     }

    //     num
    //     // 0ms/2.29MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::num_friend_requests(vec![16, 16]), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::num_friend_requests(vec![16, 17, 18]), 2);
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::num_friend_requests(vec![20, 30, 100, 110, 120]),
            3
        );
    }

    #[test]
    fn trap1() {
        assert_eq!(Solution::num_friend_requests(vec![99, 100, 110, 120]), 6);
    }

    #[test]
    fn trap2() {
        assert_eq!(Solution::num_friend_requests(vec![1, 2, 4, 16]), 0);
    }
}
