class Solution {
    fun twoSum(nums: IntArray, target: Int): IntArray {
        val numMap = HashMap<Int, Int>()

        nums.forEachIndexed { numIndex, numItem ->
            val neededValue = target - numItem
            val searchResult = numMap.get(neededValue)

            if (searchResult != null) {
                return intArrayOf(searchResult, numIndex)
            }

            numMap.put(numItem, numIndex)
        }

        return intArrayOf()
    }
}