impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if
        s.len() != t.len(){
            println!("exiting early");
            return false;
        }

        if (s.len() == 0 || s.len() == 1) ||
        (s.len() == 2 && s.chars().nth(0) == t.chars().nth(0))
        {
            println!("exiting early");
            if s.chars().nth(0) != t.chars().nth(0) {
                return false;
            }
            else {
                return true;
            }
        }
        
        let mut hash_map_s = HashMap::new();
        let mut hash_map_t = HashMap::new();

        for i in s.chars() {
            if let Some(x) = hash_map_s.get_mut(&i) {
                *x += 1;
            }
            else {
                hash_map_s.insert(i,1);
            }
        }

        for i in t.chars() {
            if let Some(x) = hash_map_t.get_mut(&i) {
                *x += 1;
            }
            else {
                hash_map_t.insert(i,1);
            }
        }

        if hash_map_t == hash_map_s {
            return true
        }
        else {
            return false;
        }



        

    }   
}
