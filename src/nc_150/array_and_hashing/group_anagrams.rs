/// Given an array of strings strs, group the anagrams together. You can return the answer in any order.
///
/// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut ans = std::collections::HashMap::new();

    for s in strs {
        let mut count = vec![0; 26];
        for c in s.chars() {
            count[c as usize - 'a' as usize] += 1;
        }
        ans.entry(count).or_insert(vec![]).push(s);
    }

    let mut result: Vec<Vec<String>> = ans.values().cloned().collect();
    result.sort();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = group_anagrams(vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ]);
        assert_eq!(
            result,
            vec![vec!["bat"], vec!["eat", "tea", "ate"], vec!["tan", "nat"]]
        );
    }

    #[test]
    fn example_2() {
        let result = group_anagrams(vec!["".to_string()]);
        assert_eq!(result, vec![vec![""]]);
    }

    #[test]
    fn example_3() {
        let result = group_anagrams(vec!["a".to_string()]);
        assert_eq!(result, vec![vec!["a"]]);
    }
}
