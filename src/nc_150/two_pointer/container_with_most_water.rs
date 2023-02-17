/*
You are given an integer array height of length n. There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).

Find two lines that together with the x-axis form a container, such that the container contains the most water.

Return the maximum amount of water a container can store.

Notice that you may not slant the container.
*/

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = height.len() - 1;
    let mut most_water = i32::MIN;

    while left < right {
        let l_height = height[left];
        let r_height = height[right];
        let volume = l_height.min(r_height) * (right - left) as i32;

        most_water = most_water.max(volume);

        if l_height < r_height {
            left += 1;
        } else {
            right -= 1;
        }
    }

    most_water
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn example_2() {
        assert_eq!(max_area(vec![1, 1]), 1);
    }
}
