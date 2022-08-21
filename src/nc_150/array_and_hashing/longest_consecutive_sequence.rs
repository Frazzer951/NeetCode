/// Given an unsorted array of integers nums, return the length of the longest consecutive elements sequence.
///
/// You must write an algorithm that runs in O(n) time.

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut num_set = std::collections::HashSet::new();
    for num in nums {
        num_set.insert(num);
    }

    let mut longest_streak = 0;

    for num in &num_set {
        if !num_set.contains(&(num - 1)) {
            let mut cur_num = *num;
            let mut cur_streak = 1;

            while num_set.contains(&(cur_num + 1)) {
                cur_num += 1;
                cur_streak += 1;
            }

            longest_streak = longest_streak.max(cur_streak);
        }
    }

    longest_streak
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = longest_consecutive(vec![100, 4, 200, 1, 3, 2]);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let result = longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]);
        assert_eq!(result, 9);
    }
}
