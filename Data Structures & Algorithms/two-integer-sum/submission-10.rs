

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 2 {
            return vec![0,1];
        }

        let mut set = HashSet::new();

        for i in &nums {
            set.insert(*i);
        }

        println!("{set:?}");

        for pos_1 in 0..nums.len() {

            let v = target - nums[pos_1];

            if set.contains(&v) {
                // THEN we can search for the other one
                for pos_2 in 0..nums.len() {
                    if nums[pos_2] == v && pos_1 != pos_2 {
                        return vec![pos_1 as i32,pos_2 as i32];
                    }
                }
            }
            else {
                continue;
            }
        }
        

        panic!("this shouldn't be possible");
    }
}
