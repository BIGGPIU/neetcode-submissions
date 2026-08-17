impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        if nums.len() == 0 || nums.len() == 1 {
            return false;
        }
        
        let mut set = HashSet::new();
        let mut last_seen_set_length = 0;

        for i in 0..nums.len() {
            set.insert(nums[i]);

            // if we successfully inserted something
            if set.len() != last_seen_set_length {
                last_seen_set_length += 1;
            }
            // if its already in there 
            else {
                return true;
            }
        }

        return false;
        
    }
}
