impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        for i in 0..nums.len() {
            sum += nums[i];
        }
        let mut left_sum = 0;

        for i in 0..nums.len() {
            if sum - left_sum - nums[i] == left_sum {
                return i as i32;
            } 
            else {
                left_sum += nums[i];
            }
        }


        return -1 as i32
    }
}
