impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut hash_map = HashMap::new();

        for i in nums {
            if hash_map.insert(i,0) != None {
                return true;
            }
        }


        return false
    }
}
