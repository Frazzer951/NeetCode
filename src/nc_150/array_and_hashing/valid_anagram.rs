pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut chars = std::collections::HashMap::new();
    s.chars().for_each(|c| *chars.entry(c).or_insert(0) += 1);
    t.chars().for_each(|c| *chars.entry(c).or_insert(0) -= 1);
    chars.into_values().all(|v| v == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = is_anagram("anagram".to_string(), "nagaram".to_string());
        assert!(result);
    }

    #[test]
    fn example_2() {
        let result = is_anagram("rat".to_string(), "car".to_string());
        assert!(!result);
    }
}
