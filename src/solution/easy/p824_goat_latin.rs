struct Solution;

impl Solution {
    /// [824. 山羊拉丁文](https://leetcode.cn/problems/goat-latin/)
    pub fn to_goat_latin(sentence: String) -> String {
        sentence
            .split_ascii_whitespace()
            .enumerate()
            .map(|(i, w)| {
                match w[..1]
                    .to_lowercase()
                    .starts_with(&['a', 'e', 'i', 'o', 'u'][..])
                {
                    true => w.to_string() + "ma" + "a".repeat(i + 1).as_str(),
                    false => w[1..].to_string() + &w[0..1] + "ma" + "a".repeat(i + 1).as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
        // 0ms/2.1MB
    }

    // pub fn to_goat_latin(mut sentence: String) -> String {
    //     sentence.push(' ');
    //     let mut goat = Vec::new();
    //     let (mut word, mut first, mut count) = (Vec::new(), ' ', 1);
    //     for c in sentence.chars() {
    //         match c {
    //             'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => word.push(c),

    //             ' ' => {
    //                 if first != ' ' {
    //                     word.push(first);
    //                 }
    //                 word.push('m');
    //                 word.push('a');
    //                 for _ in 0..count {
    //                     word.push('a');
    //                 }
    //                 word.push(c);
    //                 goat.append(&mut word);

    //                 word.clear();
    //                 first = ' ';
    //                 count += 1;
    //             }
    //             _ => {
    //                 if first == ' ' && word.is_empty() {
    //                     first = c;
    //                 } else {
    //                     word.push(c);
    //                 }
    //             }
    //         }
    //     }

    //     goat.pop();
    //     goat.iter().collect()
    //     // 0ms/2.1MB
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::to_goat_latin("I speak Goat Latin".to_string()),
            "Imaa peaksmaaa oatGmaaaa atinLmaaaaa".to_string()
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::to_goat_latin("The quick brown fox jumped over the lazy dog".to_string()),
            "heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa".to_string()
        );
    }
}
