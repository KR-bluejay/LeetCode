impl Solution {
    pub fn make_the_integer_zero(num1: i32, num2: i32) -> i32 {
        let num1 = num1 as i64;
        let num2 = num2 as i64;

        let mut op_count = 1;

        loop {
            let m = num1 - op_count * num2;

            if op_count > m {
                return -1;
            }

            if op_count >= m.count_ones() as i64 {
                return op_count as i32;
            }

            op_count += 1;
        }

    }
}