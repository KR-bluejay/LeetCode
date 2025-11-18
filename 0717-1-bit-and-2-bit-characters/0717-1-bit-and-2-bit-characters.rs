impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        if bits.len() == 1 || bits[bits.len() - 2] == 0 {
            return true;
        }
        
        let mut bit_id = 0;

        while bit_id < bits.len() {
            if bits[bit_id] == 0 {
                bit_id += 1;
            } else {
                if bit_id + 2 == bits.len() {
                    return false;
                }
                bit_id += 2;
            }
        }


        true
    }
}