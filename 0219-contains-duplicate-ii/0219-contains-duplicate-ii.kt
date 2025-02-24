class Solution {
    fun containsNearbyDuplicate(nums: IntArray, k: Int): Boolean {
        val numMap = HashMap<Int, Int>()

        nums.forEachIndexed { numIndex, numItem ->
            if (!numMap.contains(numItem)) {
                numMap.put(numItem, numIndex)
                
                return@forEachIndexed
            }


            val prevIndex = numMap.getOrDefault(numItem, 0)
            val distance = Math.abs(prevIndex - numIndex)

            if (distance <= k) {
                return true
            }

            numMap.put(numItem, numIndex)
        }

        return false
    }
}