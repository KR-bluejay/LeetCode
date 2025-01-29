class Solution {
    fun repeatedCharacter(s: String): Char {
        val letterMap = mutableMapOf<Char, Int>()

        s.forEachIndexed { letterIndex, letterItem -> 
           val letterCount = letterMap.merge(letterItem, 1, Int::plus)

           if (letterCount == 2) {
                return letterItem
           }
        } 

        return ' '
    }
}