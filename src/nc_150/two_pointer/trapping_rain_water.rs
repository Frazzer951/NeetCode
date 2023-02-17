/*
Given n non-negative integers representing an elevation map where the width of each bar is 1, compute how much water it can trap after raining.
*/

pub fn trap(height: Vec<i32>) -> i32 {
    let mut total = 0;
    let mut seen = vec![0; height.len()];
    let mut highest_left = 0;
    for i in 0..height.len() {
        highest_left = highest_left.max(height[i]);
        seen[i] = highest_left;
    }
    let mut highest_right = 0;
    for i in (0..height.len()).rev() {
        let cur_hight = height[i];
        highest_right = highest_right.max(cur_hight);

        let min_height = seen[i].min(highest_right);

        if cur_hight < min_height {
            total += min_height - cur_hight;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }

    #[test]
    fn example_2() {
        assert_eq!(trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }
}
