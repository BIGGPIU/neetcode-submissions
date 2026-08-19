impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let str_bytes:Vec<char> = s.chars().collect();

        if 
        1 >= str_bytes.len() 
        {
            return true;
        }



        let mut l_ptr = 0;
        let mut r_ptr = str_bytes.len() - 1;


        while r_ptr > l_ptr {
            

            if !str_bytes[l_ptr].is_ascii_alphanumeric() {
                l_ptr += 1;
                continue;
            }
            if !str_bytes[r_ptr].is_ascii_alphanumeric() {
                r_ptr -= 1;
                continue;
            }

            if str_bytes[l_ptr].to_lowercase().to_string() == str_bytes[r_ptr].to_lowercase().to_string() {
                l_ptr += 1;
                r_ptr -= 1;
                continue;
            }
            else {
                return false;
            }
        }



        return true
    }
}
