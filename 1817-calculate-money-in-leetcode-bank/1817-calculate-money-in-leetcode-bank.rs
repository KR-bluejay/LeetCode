impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let mut money = 0;

        for i in 0 .. n / 7 {
            money += (i + 1 .. i + 8).sum::<i32>();
        }

        if n % 7 > 0 {
            money += ((n / 7 + 1) .. (n / 7 + 1 + n % 7)).sum::<i32>();
        }

        money
    }
}