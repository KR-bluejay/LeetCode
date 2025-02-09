class Solution {
    fun tribonacci(n: Int): Int {
        var cachedList = mutableListOf<Int>(0, 1, 1)

        if (n <= 2) {
            return cachedList.get(n)
        }

        for (i in 3 .. n) {
            var currentValue = cachedList.sum()

            cachedList[0] = cachedList[1]
            cachedList[1] = cachedList[2]
            cachedList[2] = currentValue
        }

        return cachedList[2]
    }
}