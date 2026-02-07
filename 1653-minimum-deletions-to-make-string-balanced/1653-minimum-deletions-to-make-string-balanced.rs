impl Solution {
    fn count(
        s_bytes: &Vec<u8>,
        s_cache: &mut Vec<Vec<i32>>,
        byte_id: usize,
        type_id: usize,
    ) -> i32 {
        if byte_id >= s_bytes.len() {
            return 0;
        }
        if s_cache[byte_id][type_id] != i32::MAX {
            return s_cache[byte_id][type_id];
        }

        let mut count = (s_bytes[byte_id] - b'a' != type_id as u8) as i32;


        if byte_id + 1 < s_bytes.len() {
            let type_a = Self::count(
                s_bytes,
                s_cache,
                byte_id + 1,
                type_id,
            );

            let type_b = if type_id == 0 {
                Self::count(
                    s_bytes,
                    s_cache,
                    byte_id + 1,
                    type_id + 1,
                )
            } else {
                i32::MAX
            };

            count += type_a.min(type_b);
        }

        s_cache[byte_id][type_id] = count;
        s_cache[byte_id][type_id]
    }
    pub fn minimum_deletions(s: String) -> i32 {
        let s_bytes: Vec<u8> = s.into_bytes();
        let mut s_cache: Vec<Vec<i32>> = vec![vec![i32::MAX; 2]; s_bytes.len()];

        Self::count(&s_bytes, &mut s_cache, 0, 0)
            .min(Self::count(&s_bytes, &mut s_cache, 0, 1))
    }
}