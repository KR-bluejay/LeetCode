class Solution {
    fun checkIfPangram(sentence: String): Boolean {
        val alphabetSet = HashSet<Char>()

        sentence.forEach { alphabetItem -> 
            alphabetSet.add(alphabetItem)
        }

        return alphabetSet.size == 26
    }
}