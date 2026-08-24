impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() == t.len() {
            if s.len() == 1 && s == t {
                return true;
            }
            if s.len() == 2 && s.chars().nth(0) == t.chars().nth(1) {
                return true;
            }
        }
        else {
            return false;
        }



        let mut hash_map = HashMap::new();

        for i in s.chars() {
            match hash_map.get_mut(&i) {
                Some(x) => {
                    *x += 1;
                }
                None => {
                    hash_map.insert(i,1);
                }
            }
        }



        for i in t.chars() {
            match hash_map.get_mut(&i) {
                Some(x) => {
                    *x -= 1;
                    if x == &0 {
                        // I think rust wont allow this because its a drop while borrowed
                        // which iirc the compiler nono likey
                        hash_map.remove(&i);
                    }
                }
                None => {
                    return false;
                }
            }
        }



        return hash_map.len() == 0;
    }
}
