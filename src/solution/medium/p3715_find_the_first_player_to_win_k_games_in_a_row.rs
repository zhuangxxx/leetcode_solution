struct Solution;

impl Solution {
    /// [3175. 找到连续赢 K 场比赛的第一位玩家](https://leetcode.cn/problems/find-the-first-player-to-win-k-games-in-a-row/)
    pub fn find_winning_player(skills: Vec<i32>, k: i32) -> i32 {
        let mut time = 0;

        let (mut i, mut last_i) = (0, 0);
        while i < skills.len() {
            let mut j = i + 1;
            while j < skills.len() && skills[j] < skills[i] && time < k {
                j += 1;
                time += 1;
            }

            last_i = i as i32;
            if time == k {
                break;
            }

            time = 1;
            i = j;
        }

        last_i
        // 0ms/3.53MB
    }

    // pub fn find_winning_player(skills: Vec<i32>, k: i32) -> i32 {
    //     let (mut curr, mut time) = (0, 0);
    //     for i in (0..skills.len()).cycle().skip(1) {
    //         if time == k || i == curr {
    //             break;
    //         }
    //         if skills[i] > skills[curr] {
    //             curr = i;
    //             time = 1;
    //         } else {
    //             time += 1;
    //         }
    //     }

    //     curr as i32
    //     // 4ms/3.58MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::find_winning_player(vec![4, 2, 6, 3, 9], 2), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::find_winning_player(vec![2, 5, 4], 3), 1);
    }

    #[test]
    fn fail1() {
        assert_eq!(
            Solution::find_winning_player(vec![20, 16, 11, 8], 563748262),
            0
        );
    }
}
