impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut num_vec:Vec<i32> = Vec::with_capacity(operations.len());


        for i in operations {
            match i.as_str() {
                "+" => {
                    let ptr = num_vec.len() - 2;
                    
                    let new_num = num_vec[ptr] + num_vec[ptr + 1];
                    
                    num_vec.push(new_num)
                }
                "C" => {
                    num_vec.pop();
                }
                "D" => {
                    num_vec.push(num_vec[num_vec.len() - 1] * 2);
                }
                _ => {
                    // numbers
                    num_vec.push(i.parse::<i32>().unwrap());
                }
            }
        }

        let mut result = 0;

        // I could update result in the main loop but this is just an initial wraparound
        for i in num_vec {
            result += i;
        }

        return result
    }
}
