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

        // not actually unsafe because all the characters are UTF-8
        let mut s_srt:Vec<u8> = s.bytes().collect();
        let mut t_srt:Vec<u8> = t.bytes().collect();

        s_srt.sort();
        t_srt.sort();

        // println!("{s_srt:?}");
        // println!("{t_srt:?}");

        for i in 0..s_srt.len() {
            

            if s_srt[i] == t_srt[i] {
                continue;
            }
            else {
                return false;
            }
        }

        return true;
    }
}
