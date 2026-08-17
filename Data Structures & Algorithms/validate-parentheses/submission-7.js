class Solution {
    /**
     * @param {string} s
     * @return {boolean}
     */

    isValid(s) {
        let dic = {
            "[":"]",
            "{":"}",
            "(":")",

            keys() {
                return ["[","{","("]
            }
        }
        let hold = ""
        let stack = []
        for (let i = 0; i < s.length; i++) {
            if (dic.keys().includes(s[i])) {
                stack.push(s[i])
            }
            else {
                try {
                    hold = stack[stack.length-1]
                    if (s[i] == dic[hold]) {
                        stack.pop()
                    }
                    else {
                        return false
                    }
                } catch (error) {
                    return false
                }
            } 
            
        }
        return (stack.length == 0)
    }
}
