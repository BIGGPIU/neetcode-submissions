impl Solution {
    pub fn is_valid(s: String) -> bool {
        
        if 1 >= s.len() {
            return false;
        }

        
        // idk if im supposed to use VecDeque here but I don't think It'll matter
        let mut stack = Vec::with_capacity(s.len() / 2);
        let parenthesis_table:HashMap<char,char> = HashMap::from([
            ('[',']'),
            ('{','}'),
            ('(',')'),
        ]);

        


        for i in s.chars() {
            if i == '(' || i == '{' || i == '[' {
                // insert into the queue
                stack.push(parenthesis_table.get(&i));
                continue;
            }
            
            match stack.pop() {
                Some(x) => {
                    if x.unwrap() == &i {
                        // do nothing
                    } 
                    else {
                        return false;
                    }
                }
                None => {
                    return false;
                }
            }

            
        }


        return stack.len() == 0;

    }
}
