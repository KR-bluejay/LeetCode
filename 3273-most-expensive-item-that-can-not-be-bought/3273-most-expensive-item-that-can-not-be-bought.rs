impl Solution {
    pub fn most_expensive_item(prime_one: i32, prime_two: i32) -> i32 {
        let prime_one = prime_one as usize;
        let prime_two = prime_two as usize;

        let mut is_expensive_items: Vec<bool> = vec![true; prime_one * prime_two + 1];
        let item_len = prime_one * prime_two;

        
        for i in 0 ..= (item_len / prime_two) {
            let result = item_len - (prime_two * i);

            for j in 0 ..= (result / prime_one) {
                is_expensive_items[prime_one * j + prime_two * i] = false;
            }
        }

        for i in (1 .. item_len).rev() {
            if is_expensive_items[i] {
                return i as i32;
            }
        }
        -1
    }
}