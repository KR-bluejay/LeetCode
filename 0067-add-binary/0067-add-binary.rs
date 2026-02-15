impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a_bytes = a.into_bytes();
        let b_bytes = b.into_bytes();

        let mut a_id = a_bytes.len() - 1;
        let mut b_id = b_bytes.len() - 1;
        
        let mut carry = 0;

        let mut result = Vec::with_capacity(a_bytes.len().max(b_bytes.len()) * 2);


        while a_id < a_bytes.len() || b_id < b_bytes.len() || carry > 0 {
            carry += a_bytes.get(a_id).unwrap_or(&b'0') - b'0';
            carry += b_bytes.get(b_id).unwrap_or(&b'0') - b'0';

            result.push((carry % 2) + b'0');
            carry /= 2;

            a_id -= 1;
            b_id -= 1;
        }

        unsafe {String::from_utf8_unchecked(result.into_iter().rev().collect())}
    }
}