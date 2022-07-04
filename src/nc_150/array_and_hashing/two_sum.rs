pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() - 1 {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn example_2() {
        let result = two_sum(vec![3, 2, 4], 6);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn example_3() {
        let result = two_sum(vec![3, 3], 6);
        assert_eq!(result, vec![0, 1]);
    }
}
