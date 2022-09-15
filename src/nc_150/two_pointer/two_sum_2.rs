pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut l = 0;
    let mut r = numbers.len() - 1;

    loop {
        let sum = numbers[l] + numbers[r];

        match sum {
            _ if sum == target => {
                return vec![l as i32 + 1, r as i32 + 1];
            }
            _ if sum < target => {
                l += 1;
            }
            _ => {
                r -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn example_2() {
        let result = two_sum(vec![2, 3, 4], 6);
        assert_eq!(result, vec![1, 3]);
    }

    #[test]
    fn example_3() {
        let result = two_sum(vec![-1, 0], -1);
        assert_eq!(result, vec![1, 2]);
    }
}
