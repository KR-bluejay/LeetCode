class Solution {
    fun isValid(s: String): Boolean {
        val charStack = Stack<Char>()
        val charMap = hashMapOf<Char, Char>(
            '(' to ')', 
            '{' to '}', 
            '[' to ']'
        )

        for (charItem in s) {
            if (charItem == '(' || charItem == '{' || charItem == '[') {
                charStack.push(charItem)
                
                continue
            }

            val charKey = charMap.entries.firstOrNull {it.value == charItem}?.key
            val targetIndex = charStack.indexOf(charKey)

            if (charStack.isEmpty() || charStack.peek() != charKey) {
                return false
            }

            charStack.pop()
        }

        return charStack.isEmpty() 
    }
}