impl Solution {
    pub fn minimum_levels(possible: Vec<i32>) -> i32 {
        let mut prefix = Vec::with_capacity(possible.len());

        if possible[0] == 1 {
            prefix.push(1);
        } else {
            prefix.push(-1);
        }


        for (id, &item) in possible.iter().enumerate().skip(1) {
            let cur_score = prefix[id - 1] + if item == 1 {
                1
            } else {
                -1
            };

            prefix.push(cur_score);
        }

        for (id, &score) in prefix.iter().enumerate() {
            if id + 1 == prefix.len() {
                return -1;
            } else if score > *prefix.last().unwrap() - score {
                return id as i32 + 1;
            }
        }


        -1
    }
}