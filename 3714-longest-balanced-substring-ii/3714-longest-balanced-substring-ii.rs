use std::collections::HashMap;

impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        let bytes: Vec<u8> = s.into_bytes();
        
        let mut byte_map: HashMap<i32, i32> = HashMap::new();
        let mut byte_sum = 1;

        let mut result = 1;

        for (id, &byte_item) in bytes.iter().enumerate().skip(1) {
            byte_sum = if bytes[id - 1] == byte_item {
                byte_sum + 1
            } else {
                1
            };

            result = result.max(byte_sum);
        }
        
        byte_sum = 0;
        byte_map.insert(0, -1);
        
        for (id, &byte_item) in bytes.iter().enumerate() {
            if byte_item == b'a' {
                byte_sum = 0;
                byte_map.clear();
                byte_map.insert(0, id as i32);

                continue;
            }

            byte_sum += if byte_item == b'b' {
                1
            } else {
                -1
            };

            if let Some(&byte_id) = byte_map.get(&byte_sum) {
                result = result.max(id as i32 - byte_id);
            } else {
                byte_map.insert(byte_sum, id as i32);
            }
        }
        
        byte_sum = 0;
        byte_map.clear();
        byte_map.insert(0, -1);
        
        for (id, &byte_item) in bytes.iter().enumerate() {
            if byte_item == b'b' {
                byte_sum = 0;
                byte_map.clear();
                byte_map.insert(0, id as i32);

                continue;
            }

            byte_sum += if byte_item == b'a' {
                1
            } else {
                -1
            };

            if let Some(&byte_id) = byte_map.get(&byte_sum) {
                result = result.max(id as i32 - byte_id);
            } else {
                byte_map.insert(byte_sum, id as i32);
            }
        }

        byte_sum = 0;
        byte_map.clear();
        byte_map.insert(0, -1);
        for (id, &byte_item) in bytes.iter().enumerate() {
            if byte_item == b'c' {
                byte_sum = 0;
                byte_map.clear();
                byte_map.insert(0, id as i32);

                continue;
            }

            byte_sum += if byte_item == b'a' {
                1
            } else {
                -1
            };

            if let Some(&byte_id) = byte_map.get(&byte_sum) {
                result = result.max(id as i32 - byte_id);
            } else {
                byte_map.insert(byte_sum, id as i32);
            }
        }

        let mut third_map: HashMap<(i32, i32), i32> = HashMap::new();
        third_map.insert((0, 0), -1);

        let mut freq: [i32; 3] = [0, 0, 0];

        for (byte_id, &byte_item) in bytes.iter().enumerate() {
            freq[(byte_item - b'a') as usize] += 1;
            
            let map_key = (freq[0] - freq[1], freq[1] - freq[2]);

            if let Some(id) = third_map.get(&map_key) {
                result = result.max(byte_id as i32 - id);
            } else {
                third_map.insert(map_key, byte_id as i32);
            }
        }



        result
    }
}