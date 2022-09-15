pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    nums.sort();

    for (i, a) in nums.iter().enumerate() {
        if i > 0 && *a == nums[i - 1] {
            continue;
        }

        let mut l = i + 1;
        let mut r = nums.len() - 1;
        while l < r {
            let three_sum = a + nums[l] + nums[r];
            match three_sum {
                _ if three_sum > 0 => r -= 1,
                _ if three_sum < 0 => l += 1,
                _ => {
                    res.push(vec![*a, nums[l], nums[r]]);
                    l += 1;
                    while nums[l] == nums[l - 1] && l < r {
                        l += 1;
                    }
                }
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = three_sum(vec![-1, 0, 1, 2, -1, -4]);
        assert_eq!(result, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    }

    #[test]
    fn example_2() {
        let result = three_sum(vec![0, 1, 1]);
        assert!(result.is_empty());
    }

    #[test]
    fn example_3() {
        let result = three_sum(vec![0, 0, 0]);
        assert_eq!(result, vec![vec![0, 0, 0]]);
    }
}
