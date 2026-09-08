impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        // make the monotonic increasing stack
        // (make it look like this)
        let mut hash_map = HashMap::new();
        let mut result = Vec::with_capacity(nums1.len());


        for (v,k) in nums1.into_iter().enumerate() {
            hash_map.insert(k,v);
            result.push(-1);
        }

        let mut stack = VecDeque::new();

        for i in 0..nums2.len() {
            // if nums2[i] is greater than the top of the stack
            while (stack.len() != 0 && nums2[i] >= stack[0]) {
               // stack.len() cannot equal 0
               
               // pop the top of the stack
               let x = stack.pop_front().unwrap();

               // and modify the result (if relevant)
               if let Some(idx) = hash_map.get(&x) {
                    result[*idx] = nums2[i];
               }
            }

            stack.push_front(nums2[i]);
        }

        return result
    }
}
