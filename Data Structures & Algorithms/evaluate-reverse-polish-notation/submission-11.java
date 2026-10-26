class Solution {
    public int evalRPN(String[] tokens) {
        ArrayDeque<Integer> stack = new ArrayDeque();

        for (int idx = 0; tokens.length > idx; idx++) {
            String c = tokens[idx];

            if (c.equals("+")) {
                int value_2 = stack.removeLast();
                int value_1 = stack.removeLast();

                stack.add(value_1 + value_2);
            }
            else if (c.equals("-")) {
                int value_2 = stack.removeLast();
                int value_1 = stack.removeLast();

                stack.add(value_1 - value_2);
            }
            else if (c.equals("*")) {
                int value_2 = stack.removeLast();
                int value_1 = stack.removeLast();

                stack.add(value_1 * value_2);
            }
            else if (c.equals("/")) {
                int value_2 = stack.removeLast();
                int value_1 = stack.removeLast();

                stack.add(value_1 / value_2);
            }
            else {
                stack.add(Integer.parseInt(c));
            }

            
        }


        return stack.pop();
    }
}
