pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut res: Vec<i32> = vec![-1, -1];
    if nums.len() == 0 {
        return res;
    }
    let mut left: i32 = 0;
    let mut right: i32 = (nums.len() - 1) as i32;

    // first occurence
    while left <= right {
        let mid = left + ((right - left) / 2);

        if nums[mid as usize] == target {
            res[0] = mid as i32;
            right = mid - 1;
        } else if nums[mid as usize] <= target {
            left = mid + 1;
        } else if nums[mid as usize] > target {
            right = mid - 1;
        }
    }

    left = 0;
    right = (nums.len() - 1) as i32;
    // last occurence
    while left <= right {
        let mid = left + ((right - left) / 2);

        if nums[mid as usize] == target {
            res[1] = mid as i32;
            left = mid + 1;
        } else if nums[mid as usize] <= target {
            left = mid + 1;
        } else if nums[mid as usize] > target {
            right = mid - 1;
        }
    }

    res
}
