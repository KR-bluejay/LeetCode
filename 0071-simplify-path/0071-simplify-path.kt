class Solution {
    fun simplifyPath(path: String): String {
        val directoryList = path.split('/')
        val directoryDeque = ArrayDeque<String>()

        directoryList.forEach {
            // 공백인 경우 skip
            if (it == "" || it == ".") {
                return@forEach
            }

            if (it == "..") {
                directoryDeque.removeLastOrNull()

                return@forEach
            }

            directoryDeque.add(it)
        }

        return "/" + directoryDeque.toArray().joinToString("/")
    }
}