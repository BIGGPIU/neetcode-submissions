impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut l_ptr = 0;
        let mut r_ptr = numbers.len() - 1;

        loop {
            if numbers[l_ptr] + numbers[r_ptr] == target {
                return vec![(l_ptr + 1) as i32,(r_ptr + 1) as i32];
            }
            else if numbers[l_ptr] + numbers[r_ptr] > target {
                r_ptr -= 1;
            }
            else {
                l_ptr += 1;
            }
        }

    }
}
