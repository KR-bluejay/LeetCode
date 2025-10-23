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
        let mut nums: Vec<u8> = s.into_bytes().into_iter().map(|v| v - b'0').collect();
        let mut best_num = nums.clone();

        let step_len = 10 / Self::gcd(10.max(a as usize), 10.min(a as usize));
        let rotation_len = nums.len() / Self::gcd(nums.len(), b);
        
        for _ in 0 .. rotation_len {
            for even_step in 0 .. step_len {
                let mut cur_num = nums.clone();

                if cur_num < best_num {
                    best_num = cur_num.clone();
                }
                for cur_id in (0 .. cur_num.len())
                    .skip(1)
                    .step_by(2) {
                    
                    cur_num[cur_id] = (cur_num[cur_id] + ((a * even_step as u8) % 10 as u8)) % 10;
                }
                if b % 2 == 1 {
                    if cur_num < best_num {
                        best_num = cur_num.clone();
                    }

                    for odd_step in 0 .. step_len {
                        for odd_cur_id in (0 .. cur_num.len())
                            .step_by(2) {
                            cur_num[odd_cur_id] = (cur_num[odd_cur_id] + a as u8) % 10;
                        }
                        
                        if cur_num < best_num {
                            best_num = cur_num.clone();
                        }
                    }
                }
                if cur_num < best_num {
                    best_num = cur_num.clone();
                }
            }
            nums.rotate_right(b);
        }
        
        best_num.into_iter().map(|v| (v + b'0') as char).collect()
    }
}