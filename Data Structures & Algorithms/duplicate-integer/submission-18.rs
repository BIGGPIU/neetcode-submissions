impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut hash_set = HashSet::new();


        for i in nums {
            let len = hash_set.len();
            hash_set.insert(i);

            if hash_set.len() != len {
                // do nothing
            }
            else {
                return true;
            }
        }


        return false
    }
}
