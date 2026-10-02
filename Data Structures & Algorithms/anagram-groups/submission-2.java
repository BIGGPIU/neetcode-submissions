class Solution {
    public List<List<String>> groupAnagrams(String[] strs) {
        // <Sorted Version, List>
        HashMap<String,List<String>> stringList = new HashMap<>();


        for (String s : strs) {
            char[] chars = s.toCharArray();
            Arrays.sort(chars);
            
            String sortedString = new String(chars);

            List<String> val = stringList.get(sortedString);

            if (val == null) {
                stringList.put(sortedString,
                new ArrayList<>(Arrays.asList(s))
                );
            }
            else {
                val.add(s);
                stringList.put(sortedString,val);
            }
        }


        return List.copyOf(stringList.values());
    }
}
