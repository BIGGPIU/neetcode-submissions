impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut hash_map:HashMap<char,u32> = HashMap::new();

        for i in s.chars() {
            if let Some(x) = hash_map.get_mut(&i) {
                *x += 1;
            }
            else {
                hash_map.insert(i,1);
            }
        }



        for i in t.chars() {
            if let Some(x) = hash_map.get_mut(&i) {
                *x -= 1;

                if *x == 0 {
                    hash_map.remove(&i);
                }
            }
            else {
                return false;
            }
        }

        if hash_map.len() != 0 {
            return false;
        }


        return true;
    }
}
