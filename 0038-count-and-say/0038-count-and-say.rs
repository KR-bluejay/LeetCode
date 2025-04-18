impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return String::from("1")
        }

        let mut cnt = 2; 
        let mut cur = vec!['1'; 2];

        let mut temp = Vec::new();

        while cnt < n {
            let mut rle_count = 1;

            for i in 1 .. cur.len() {
                if cur[i - 1] == cur[i] {
                    rle_count += 1;
                } else {
                    temp.push(char::from_digit(rle_count as u32, 10).unwrap());
                    temp.push(cur[i - 1]);
                    rle_count = 1;
                }
            }
            temp.push(char::from_digit(rle_count as u32, 10).unwrap());
            temp.push(cur[cur.len() - 1]);


            std::mem::replace(&mut cur, temp);
            temp = Vec::new();

            cnt += 1;
        }

        cur.iter().collect()
    }
}