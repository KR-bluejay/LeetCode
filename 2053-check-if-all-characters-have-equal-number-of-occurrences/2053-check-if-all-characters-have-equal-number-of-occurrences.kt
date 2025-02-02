class Solution {
    fun areOccurrencesEqual(s: String): Boolean {
        val charMap = s.toList().groupingBy {it}.eachCount()
        var commonLetterCount = 0

        for ((key, value) in charMap) {
            if (commonLetterCount == 0) {
                commonLetterCount = value
            }

            if (value != commonLetterCount)  {
                return false
            }
        }

        return true
    }
}