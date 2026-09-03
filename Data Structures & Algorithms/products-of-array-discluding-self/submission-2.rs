impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut left_side_solutions = Vec::new();
        let mut right_side_solutions = Vec::new();

        let mut result = Vec::new();

        left_side_solutions.push(1);
        right_side_solutions.push(1);


        for i in 0..nums.len() {
           left_side_solutions.push(nums[i] * left_side_solutions[i]);
        }

        for i in 0..nums.len() {
            right_side_solutions.push(nums[nums.len() - 1 - i] * right_side_solutions[i]);
        }

        for i in 0..nums.len() {
            result.push(left_side_solutions[i] * right_side_solutions[nums.len() - 1 - i]);
        }

        result
    }
}
