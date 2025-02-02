class Solution {
    fun singleNumber(nums: IntArray): Int {
        var uniqueNum = 0

        for (numItem in nums) {
            uniqueNum = numItem xor uniqueNum
        }

        return uniqueNum
    }
}