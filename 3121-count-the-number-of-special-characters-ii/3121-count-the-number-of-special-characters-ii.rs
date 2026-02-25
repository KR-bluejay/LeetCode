impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut record: Vec<u8> = vec![0; 26];

        let mut count = 0;

        for word in word.into_bytes() {
            if word < b'a' {
                let key = (word - b'A') as usize;

                if record[key] == 0 {
                    record[key] = 3;
                } else if record[key] == 1 {
                    count += 1;
                    record[key] = 2;
                }
            } else {
                let key = (word - b'a') as usize;

                if record[key] == 0 {
                    record[key] = 1;
                } else if record[key] == 2 {
                    count -= 1;
                    record[key] = 3;
                }
            }
        }

        count
    }
}