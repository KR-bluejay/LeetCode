impl Solution {
    fn get_score(
        points: &mut Vec<Vec<i32>>,
        stay_score: &Vec<Vec<i32>>,
        travel_score: &Vec<Vec<i32>>,
        day_id: usize,
        city_id: usize,
    ) -> i32 {
        if points[day_id][city_id] > 0 {
            return points[day_id][city_id];
        }
        
        let mut best_score = stay_score[day_id][city_id] 
            + if day_id + 1 < stay_score.len() {
                Self::get_score(
                    points, 
                    stay_score, 
                    travel_score, 
                    day_id + 1, 
                    city_id
                )
            } else {
                0
            };
        
        for next_city_id in 0 .. stay_score[0].len() {
            if city_id == next_city_id {
                continue;
            }

            let next_score = if (day_id + 1) < stay_score.len() { 
                Self::get_score(
                    points, 
                    stay_score, 
                    travel_score, 
                    day_id + 1, 
                    next_city_id
                )
            } else {
                0
            };
            best_score = best_score.max(travel_score[city_id][next_city_id] + next_score);
        }
        
        points[day_id][city_id] = best_score;
        points[day_id][city_id]
    }
    pub fn max_score(
        n: i32, 
        k: i32, 
        stay_score: Vec<Vec<i32>>, 
        travel_score: Vec<Vec<i32>>
    ) -> i32 {
        let total_city = n as usize;
        let total_day = k as usize;

        let mut points: Vec<Vec<i32>> 
            = vec![vec![0; total_city]; total_day];

        let mut best_score = 0;

        for city_id in 0 .. stay_score[0].len() {
            best_score = best_score.max(
                Self::get_score(
                    &mut points, 
                    &stay_score, 
                    &travel_score, 
                    0, 
                    city_id
                )
            );
        }

        best_score
    }
}