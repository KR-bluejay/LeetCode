impl Solution {
    pub fn flower_game(n: i32, m: i32) -> i64 {
        let mut first_flower = n as i64;
        let mut second_flower = m as i64;

        let mut total_win: i64 = 0;
        
        total_win += (2 ..= second_flower).filter(|i| i % 2 == 0).count() as i64;

        for i in 2 ..= first_flower {
            total_win += if i % 2 == 0 {
                second_flower - (second_flower / 2)
            } else {
                second_flower / 2
            }
        }


        total_win
    }
}