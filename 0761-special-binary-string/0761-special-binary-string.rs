impl Solution {
    pub fn make_largest_special(s: String) -> String {
        let s_bytes = s.as_bytes();
        let mut specials: Vec<String> = Vec::new();
        let mut count = 0;
        let mut start_id = 0;

        for id in 0 .. s_bytes.len() {
            count += (s_bytes[id] == b'1') as i32 * 2 - 1;

            if count == 0  {
                let inner = if start_id + 1 < id {
                    Self::make_largest_special(s[start_id + 1 .. id].to_string())
                } else {
                    String::new()
                };
                
                specials.push(format!("1{inner}0"));
                start_id = id + 1;
            }
        }

        specials.sort_unstable_by(|left, right| left.cmp(&right).reverse());
        specials.join("")
    }
}