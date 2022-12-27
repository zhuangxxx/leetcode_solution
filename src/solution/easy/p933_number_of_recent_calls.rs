/// [933. 最近的请求次数](https://leetcode.cn/problems/number-of-recent-calls/)
struct RecentCounter {
    deq: std::collections::VecDeque<i32>,
    // 36ms/6.5MB
}

impl RecentCounter {
    fn new() -> Self {
        RecentCounter {
            deq: std::collections::VecDeque::new(),
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        while let Some(ping) = self.deq.front() {
            if ping < &(t - 3000) {
                self.deq.pop_front();
            } else {
                break;
            }
        }

        self.deq.push_back(t);

        self.deq.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut recent_counter = RecentCounter::new();
        assert_eq!(recent_counter.ping(1), 1);
        assert_eq!(recent_counter.ping(100), 2);
        assert_eq!(recent_counter.ping(3001), 3);
        assert_eq!(recent_counter.ping(3002), 3);
    }
}
