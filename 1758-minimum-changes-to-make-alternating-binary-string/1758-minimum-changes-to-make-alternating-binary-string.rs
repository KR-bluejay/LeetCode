impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let s = s.into_bytes();

        let mut first = Vec::with_capacity(s.len());
        let mut second = Vec::with_capacity(s.len());

        for id in 0 .. s.len() {
            if id % 2 == 0 {
                first.push(b'0');
                second.push(b'1');
            } else {
                first.push(b'1');
                second.push(b'0');
            }
        }

        let mut count_first = 0;
        let mut count_second = 0;

        for (id, s_byte) in s.into_iter().enumerate() {
            count_first += (s_byte != first[id]) as i32;
            count_second += (s_byte != second[id]) as i32;
        }

        count_first.min(count_second)
    }
}