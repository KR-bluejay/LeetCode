use std::collections::BTreeSet;

impl Solution {
    #[inline]
    fn gcd(mut m: usize, mut n: usize) -> usize {
        while n != 0 {
            let r = m % n;
            m = n;
            n = r;
        }
        m
    }
    
    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        let a = a as u8;
        let b = b as usize;
        let mut nums: Vec<u8> = s.bytes().map(|v| v - b'0').collect();
        let mut best = nums.clone();
        
        let step_len = 10 / Self::gcd(10, a as usize);
        let rotation_len = nums.len() / Self::gcd(nums.len(), b);
        let is_b_odd = b % 2 == 1;
        
        for _ in 0..rotation_len {
            // Try all odd index modifications
            for odd_step in 0..step_len {
                let odd_add = (a * odd_step as u8) % 10;
                
                // Apply odd modification
                for i in (1..nums.len()).step_by(2) {
                    nums[i] = (nums[i] + odd_add) % 10;
                }
                
                if nums < best {
                    best.copy_from_slice(&nums);
                }
                
                // If b is odd, we can also modify even indices
                if is_b_odd {
                    for even_step in 0..step_len {
                        let even_add = (a * even_step as u8) % 10;
                        
                        // Apply even modification
                        for i in (0..nums.len()).step_by(2) {
                            nums[i] = (nums[i] + even_add) % 10;
                        }
                        
                        if nums < best {
                            best.copy_from_slice(&nums);
                        }
                        
                        // Revert even modification (subtract is same as add complement)
                        let even_revert = (10 - even_add) % 10;
                        for i in (0..nums.len()).step_by(2) {
                            nums[i] = (nums[i] + even_revert) % 10;
                        }
                    }
                }
                
                // Revert odd modification
                let odd_revert = (10 - odd_add) % 10;
                for i in (1..nums.len()).step_by(2) {
                    nums[i] = (nums[i] + odd_revert) % 10;
                }
            }
            
            nums.rotate_right(b);
        }
        
        best.into_iter().map(|v| (v + b'0') as char).collect()
    }
}