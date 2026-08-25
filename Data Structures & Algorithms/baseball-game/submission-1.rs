impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut num_vec:Vec<i32> = Vec::with_capacity(operations.len());
        let mut result = 0;

        for i in operations {
            match i.as_str() {
                "+" => {
                    let ptr = num_vec.len() - 2;
                    
                    let new_num = num_vec[ptr] + num_vec[ptr + 1];
                    
                    num_vec.push(new_num);
                    // in the example where you have 
                    // [1,2, + ]
                    // it results in 1,2,3
                    // which is practically 3 * 2 
                    result += new_num
                }
                "C" => {
                    println!("{num_vec:?}");
                    let x = num_vec.pop().unwrap();
                    result -= x;
                }
                "D" => {
                    let x = num_vec[num_vec.len() - 1] * 2;
                    num_vec.push(x);
                    result += x ;
                }
                _ => {
                    // numbers
                    let x = i.parse::<i32>().unwrap();
                    num_vec.push(x);
                    result += x;
                }
            }

            println!("{result}");
        }

        return result
    }
}
