impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut depth = 0;

        for i in logs {
            match i.as_str() {
                "../" => {
                    depth = std::cmp::max(0,depth-1)
                }
                "./" => {
                    // do nothing
                }
                _ => {
                    depth += 1;
                }
            }

            println!("{depth:?}");
        }


        return depth
    }
}
