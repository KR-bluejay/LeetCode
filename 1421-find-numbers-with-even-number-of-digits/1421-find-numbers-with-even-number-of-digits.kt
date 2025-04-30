class Solution {
    fun findNumbers(nums: IntArray): Int {
        var numCount = 0

        for (numItem in nums) {
            if (((numItem.toString()).length) % 2 == 0)  {
                numCount += 1
            }
        }
        return numCount
    }
}