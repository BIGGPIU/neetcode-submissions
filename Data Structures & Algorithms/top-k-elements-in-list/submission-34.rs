impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut hashmap:HashMap<i32,i32> = HashMap::new();

        for i in nums {
            if let Some(x) = hashmap.get_mut(&i) {
                *x += 1;
            }
            else {
                hashmap.insert(i,1);
            }
        }

        let mut end_vec:Vec<(i32,i32)> = hashmap.iter().map(
            |(k,v)| (*k,*v)
        ).collect();

        end_vec.sort_unstable_by_key(|k| k.1);

        // println!("{end_vec:?}");

        let mut actual_final_vec = Vec::new();

        for i in 0..(k as usize) {
            actual_final_vec.push(end_vec[end_vec.len() - (i + 1)].0);
        }


        return actual_final_vec
        
        
    }
}
