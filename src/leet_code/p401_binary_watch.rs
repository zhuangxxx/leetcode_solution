struct Solution;

impl Solution {
    /// [401. 二进制手表](https://leetcode.cn/problems/binary-watch/)
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let (turned_on, mut times) = (turned_on as u32, Vec::new());

        for hour in 0..12i32 {
            let hour_ones = hour.count_ones();
            if hour_ones == turned_on {
                times.push(format!("{hour}:00"));
            } else if hour_ones < turned_on {
                for minute in 0..60i32 {
                    if hour_ones + minute.count_ones() == turned_on {
                        times.push(format!("{}:{:02}", hour, minute));
                    }
                }
            }
        }

        times
        // 0ms/2.2MB
    }

    // pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
    //     let (turned_on, mut times) = (turned_on as u32, Vec::new());

    //     for time in 0..1024i32 {
    //         let (hour, minute) = (time >> 6, time & 63);
    //         if hour < 12 && minute < 60 && hour.count_ones() + minute.count_ones() == turned_on {
    //             times.push(format!("{}:{:02}", hour, minute));
    //         }
    //     }

    //     times
    //     // 0ms/2.2MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::read_binary_watch(1),
            vec![
                "0:01".to_string(),
                "0:02".to_string(),
                "0:04".to_string(),
                "0:08".to_string(),
                "0:16".to_string(),
                "0:32".to_string(),
                "1:00".to_string(),
                "2:00".to_string(),
                "4:00".to_string(),
                "8:00".to_string()
            ]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::read_binary_watch(9), Vec::<String>::new());
    }
}
