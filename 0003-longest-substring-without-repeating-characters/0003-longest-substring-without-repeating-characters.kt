class Solution {
    fun lengthOfLongestSubstring(s: String): Int {
        var maxLength = 0
        var leftIndex = 0
        var rightIndex = 0

        val charMap = HashMap<Char, Int>()

        while (rightIndex < s.length) {
            val currentChar = s.get(rightIndex)
            var currentCharCount = charMap.merge(currentChar, 1, Integer::sum)!!


            while (currentCharCount > 1) {
                var leftCharCount = charMap.getOrDefault(s.get(leftIndex), 0)
                charMap.put(s.get(leftIndex), leftCharCount - 1)
                leftIndex++
                currentCharCount = charMap.getOrDefault(currentChar, 0)
            }

            maxLength = Math.max(maxLength, rightIndex - leftIndex + 1)
            rightIndex++
        }

        return maxLength
    }
}