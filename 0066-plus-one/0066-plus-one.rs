impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut prev_num = 1;

        for digit_num in digits.iter_mut().rev() {
            *digit_num += prev_num;
            prev_num = *digit_num / 10;
            *digit_num %= 10;
        }

        if prev_num > 0 {
            digits.insert(0, prev_num);
        }
        digits
    }
}