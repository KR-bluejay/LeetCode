impl Solution {
    pub fn max_bottles_drunk(mut num_bottles: i32, mut num_exchange: i32) -> i32 {
        let mut drink_count = num_bottles;
        let mut empty_count = num_bottles;

        num_bottles = 0;
        

        while empty_count + num_bottles >= num_exchange {
            if num_bottles > 0 {
                empty_count += num_bottles;
                drink_count += num_bottles;
                num_bottles = 0;
            }

            empty_count -= num_exchange;
            num_exchange += 1;
            num_bottles += 1;
        }

        drink_count + num_bottles
    }
}