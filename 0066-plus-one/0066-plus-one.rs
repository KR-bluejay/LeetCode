impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut prefix = 1;

        for digit in digits.iter_mut().rev() {
            *digit += prefix;
            prefix = *digit / 10;
            *digit %= 10;

            if prefix == 0 {
                break;
            }
        }

        if prefix > 0 {
            digits.insert(0, prefix);
        }

        digits
    }
}