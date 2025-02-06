class Solution {
    fun lengthOfLongestSubstring(s: String): Int {
        var leftIndex = 0
        var maxLength = 0

        val charMap = HashMap<Char, Int>()

        s.forEachIndexed { index, charKey ->
            if (charMap.contains(charKey)) {
                leftIndex = Math.max(charMap.getOrDefault(charKey, 0) + 1, leftIndex)
            }

            maxLength = Math.max(maxLength, index - leftIndex + 1)
            charMap.put(charKey, index)
        } 

        return maxLength
    }
}