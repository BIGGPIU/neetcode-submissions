class Solution {
    public boolean isAnagram(String s, String t) {
        HashMap<Character,Integer> anagramMap = new HashMap<>();

        for (char c : s.toCharArray()) {
            Integer val = anagramMap.get(c);

            if (val == null) {
                anagramMap.put(c,1);
            }
            else {
                anagramMap.put(c,val+1);
            }
        }   


        for (char c : t.toCharArray()) {
            Integer val = anagramMap.getOrDefault(c,-1);

            if (val == -1) {
                return false;
            }
            
            val -= 1;

            if (val == 0) {
                anagramMap.remove(c);
            }
            else {
                anagramMap.put(c,val);
            }
        }


        return (anagramMap.size() == 0);
    }
}
