impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 2 {
            return vec![0,1]
        }


        let mut map = HashMap::new();


        for (index,element) in nums.iter().enumerate() {
            map.insert(element,index);
        }

        for (index,element) in nums.iter().enumerate() {
            if let Some(v) = map.get(&(target - element)) {
                if *v != index {
                    return vec![
                        index as i32,
                        *v as i32
                    ] 
                }
                else {
                    continue;
                }
            }
        }


        panic!("");
    }
}
