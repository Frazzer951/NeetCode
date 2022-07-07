/// Given an integer array nums and an integer k, return the k most frequent elements. You may return the answer in any order.

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut frequencies = std::collections::HashMap::new();
    for n in nums {
        *frequencies.entry(n).or_insert(0) += 1;
    }
    let mut freq_vec: Vec<_> = frequencies.iter().collect();
    freq_vec.sort_by(|a, b| b.1.cmp(a.1));
    freq_vec[0..k as usize].to_vec().iter().map(|e| *e.0).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn example_2() {
        let result = top_k_frequent(vec![1], 1);
        assert_eq!(result, vec![1]);
    }
}
