impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut fancy_string: String = String::with_capacity(s.len());
        let mut s_iter = s.chars();

        let mut prev_string = s_iter.nth(0).unwrap();
        let mut prev_count = 1;

        fancy_string.push(prev_string);

        for s_item in s_iter {
            if s_item != prev_string {
                prev_string = s_item;
                prev_count = 1;

                fancy_string.push(s_item);
                
                continue;
            }

            prev_count += 1;
            prev_string = s_item;

            if prev_count >= 3 {
                prev_count -= 1;

                continue;
            }

            fancy_string.push(s_item)
        }
        fancy_string
    }
}