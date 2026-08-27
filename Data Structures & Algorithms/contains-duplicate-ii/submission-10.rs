impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        if nums.len() == 1 {
            return false;
        }
        if k== 0 {
            return false;
        }
        
        let mut hash_set = HashSet::new();
        let mut l_ptr = 0;
        let mut r_ptr = 1;

        hash_set.insert(nums[l_ptr]);

        for i in &nums {
            if hash_set.contains(&nums[r_ptr]) {
                // we've found our duplicate so return true
                return true;
            }
            else {

                hash_set.insert(nums[r_ptr]);
                r_ptr += 1;

                if r_ptr == nums.len() {
                    break;
                }

                if hash_set.len() as i32 > k  {
                    hash_set.remove(&nums[l_ptr]);
                    l_ptr += 1;
                }
            }
        }


        return false
    }
}
