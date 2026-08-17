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

        let mut hash_map = HashMap::new();


        for i in s.chars() {
            if let Some(x) = hash_map.get_mut(&i) {
                *x += 1;
            }
            else {
                hash_map.insert(i,1);
            }
        }

        let original_values:Vec<_> = hash_map.values().copied().collect();

        for i in t.chars() {
            if let Some(x) = hash_map.get_mut(&i) {
                *x += 1
            }
            else {
                return false
            }
        }

        let new_values:Vec<_> = hash_map.values().copied().collect();

        if new_values.len() != original_values.len() {
            return false;
        }
        else {
            for i in 0..new_values.len() {
                if new_values[i]/2 != original_values[i] {
                    return false;
                }
            }
        }

        return true;
    }
}
