struct Solution;

impl Solution {
    /// [332. 重新安排行程](https://leetcode.cn/problems/reconstruct-itinerary/)
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        fn bt(
            prev: String,
            map: &mut std::collections::HashMap<String, Vec<String>>,
            itinerary: &mut Vec<String>,
        ) {
            while let Some(next) = map.get_mut(&prev).unwrap_or(&mut vec![]).pop() {
                bt(next.clone(), map, itinerary);
            }
            itinerary.push(prev.clone());
        }

        let mut itinerary = Vec::new();

        let mut map = std::collections::HashMap::new();
        for ticket in tickets {
            map.entry(ticket[0].clone())
                .or_insert(Vec::new())
                .push(ticket[1].clone());
        }
        for (_, tickets) in map.iter_mut() {
            tickets.sort_by(|a, b| b.cmp(a));
        }

        bt("JFK".to_string(), &mut map, &mut itinerary);
        itinerary.reverse();

        itinerary
        // 3ms/2.23MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_itinerary(vec![
                vec!["MUC".to_string(), "LHR".to_string()],
                vec!["JFK".to_string(), "MUC".to_string()],
                vec!["SFO".to_string(), "SJC".to_string()],
                vec!["LHR".to_string(), "SFO".to_string()]
            ]),
            vec![
                "JFK".to_string(),
                "MUC".to_string(),
                "LHR".to_string(),
                "SFO".to_string(),
                "SJC".to_string()
            ]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::find_itinerary(vec![
                vec!["JFK".to_string(), "SFO".to_string()],
                vec!["JFK".to_string(), "ATL".to_string()],
                vec!["SFO".to_string(), "ATL".to_string()],
                vec!["ATL".to_string(), "JFK".to_string()],
                vec!["ATL".to_string(), "SFO".to_string()]
            ]),
            vec![
                "JFK".to_string(),
                "ATL".to_string(),
                "JFK".to_string(),
                "SFO".to_string(),
                "ATL".to_string(),
                "SFO".to_string()
            ]
        );
    }
}
