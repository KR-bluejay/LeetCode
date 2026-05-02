impl Solution {
    pub fn rotated_digits(n: i32) -> i32 {
        let mut result = 0;

        'outer: for i in 1 ..= n {
            let mut must_digit = false;
            let mut num = i;

            while num > 0 {
                let t = num % 10;

                if t == 3 || t == 4 || t == 7 {
                    continue 'outer;
                }

                must_digit |= (t == 2 || t == 5 || t == 6 || t == 9);
                num /= 10;
            }

            if must_digit {
                result += 1;
            }
        }


        result
    }
}