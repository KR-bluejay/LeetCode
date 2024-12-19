
class Solution {
public:
    bool isPalindrome(int x) {
        string xStr = to_string(x);

        for (int i = 0; i < xStr.size(); i++) {
            if (xStr.at(i) != xStr.at(xStr.size() - 1 - i)) {
                return false;
            }
        }

        return true;
    }
};