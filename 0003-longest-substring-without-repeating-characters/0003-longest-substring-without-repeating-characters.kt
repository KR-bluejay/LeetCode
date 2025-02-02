class Solution {
    fun lengthOfLongestSubstring(s: String): Int {
        var leftIndex = 0 
        var rightIndex = 0
        var maxLength = 0

        val charMap = HashMap<Char, Int>()

        while (rightIndex < s.length) {
            val charKey: Char = s[rightIndex]
            val charCount: Int = charMap.merge(charKey, 1, Integer::sum)!!

            while (charMap.getOrDefault(charKey, 0) > 1) {
                charMap.put(s[leftIndex], charMap.getOrDefault(s[leftIndex], 0) - 1)

                leftIndex += 1
            }

            maxLength = Math.max(maxLength, rightIndex - leftIndex + 1)
            rightIndex += 1
        }

        return maxLength
    }
}