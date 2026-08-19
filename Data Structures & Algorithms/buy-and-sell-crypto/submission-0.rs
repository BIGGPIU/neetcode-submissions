impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {    
        // take the first value as the minimum

        // loop through until we find something higher or lower than the minimum

        // if its lower 
            // take that as the new minimum value


        // if its higher
            // set that as our new highest sell value (if its bigger ofc)

        // continue until we reach the end of the list

        let mut minimum = prices[0];
        let mut max_sell_value = 0;

        for i in prices {
            if minimum >= i {
                minimum = i;
                continue;
            }
            else if i >= minimum{
                max_sell_value = std::cmp::max(i - minimum, max_sell_value);
            }
        }

        return max_sell_value;
    }
}
