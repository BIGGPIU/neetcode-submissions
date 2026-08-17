class Solution {
    /**
     * @param {number[]} numbers
     * @param {number} target
     * @return {number[]}
     */
    twoSum(numbers, target) {
        let x = 0 
        let y = 1
        let hold = 0 
        while (true) {
            hold = numbers[x] + numbers[y]
            if (hold == target) {
                return [x+1,y+1]
            }
            else if (hold > target) {
                x++
                y=x+1
            }
            else {
                y++
                if (y==numbers.length) {
                    x++
                    y=x+1
                }
            }
        }
    };
};
