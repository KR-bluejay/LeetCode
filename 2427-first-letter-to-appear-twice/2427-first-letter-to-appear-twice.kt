class Solution {
    fun repeatedCharacter(s: String): Char {
        val letterSet = HashSet<Char>()

        s.forEach { letterItem ->
            if (letterSet.contains(letterItem)) {
                return letterItem
            }
            letterSet.add(letterItem)
        }

        return ' '
    }
}