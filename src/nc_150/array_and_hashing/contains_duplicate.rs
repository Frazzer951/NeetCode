use std::collections::HashSet;

fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut seen = HashSet::new();

    for num in nums {
        if seen.contains(&num) {
            return true;
        }
        seen.insert(num);
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = contains_duplicate(vec![1, 2, 3, 1]);
        assert!(result);
    }

    #[test]
    fn example_2() {
        let result = contains_duplicate(vec![1, 2, 3, 4]);
        assert!(!result);
    }

    #[test]
    fn example_3() {
        let result = contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]);
        assert!(result);
    }
}
