use std::collections::HashMap;

impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        const CHAR_PAIRS: [(u8, u8, u8); 3] = [
            (b'a', b'b', b'c'), 
            (b'b', b'c', b'a'),
            (b'c', b'a', b'b')
        ];
        
        let bytes = s.into_bytes();

        let mut longest_len = 1;
        let mut two_pair_map: HashMap<i32, i32> = HashMap::with_capacity(bytes.len());

        two_pair_map.insert(0, -1);

        let mut one_pair_len = 1;

        for byte_id in 1 .. bytes.len() {
            if bytes[byte_id - 1] == bytes[byte_id] {
                one_pair_len += 1;
            } else {
                one_pair_len = 1;
            }

            longest_len = longest_len.max(one_pair_len);
        }




        for &(pair_1, pair_2, wall) in CHAR_PAIRS.iter() {
            let mut pair_sum = 0;

            for (id, &b) in bytes.iter().enumerate() {
                if b == wall {
                    pair_sum = 0;

                    two_pair_map.clear();
                    two_pair_map.insert(0, id as i32);

                    continue;
                }

                if b == pair_1 {
                    pair_sum += 1; 
                } else {
                    pair_sum -= 1;
                }

                if let Some(pair_id) = two_pair_map.get(&pair_sum) {
                    longest_len = longest_len.max(id as i32 - pair_id);
                } else {
                    two_pair_map.insert(pair_sum, id as i32);
                }
            }

            two_pair_map.clear();
            two_pair_map.insert(0, -1);
        }

        let mut pair_counts: [i32; 3] = [0, 0, 0];
        let mut third_pair_map: HashMap<(i32, i32, i32), i32> = HashMap::with_capacity(bytes.len());

        third_pair_map.insert((0, 0, 0), -1);

        for (id, &b) in bytes.iter().enumerate() {
            pair_counts[(b - b'a') as usize] += 1;

            let pair_key = (
                pair_counts[0] - pair_counts[1], 
                pair_counts[1] - pair_counts[2], 
                pair_counts[2] - pair_counts[0],
            );

            if let Some(pair_id) = third_pair_map.get(&pair_key) {
                longest_len = longest_len.max(id as i32 - pair_id);
            } else {
                third_pair_map.insert(pair_key, id as i32);
            }
        }

        longest_len
    }
}