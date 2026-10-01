impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        match nums.binary_search(&target) {
            Ok(x) => {
                return x as i32
            }
            Err(e) => {
                return -1;
            }
        }
    }
}
