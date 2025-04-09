impl Solution {
    fn backtrack(day_id: usize, days: &Vec<i32>, costs: &Vec<i32>, cache: &mut Vec<i32>) -> i32 {
        if day_id >= days.len() {
            return 0;
        }

        if cache[day_id] != -1 {
            return cache[day_id];
        }


        let cur_day = days[day_id];
        let seven_day = days[day_id] + 7;
        let month_day = days[day_id] + 30;

        let mut seven_day_id = days.len();
        let mut month_day_id = days.len();



        for i in day_id + 1 .. days.len() {
            if days[i] >= seven_day {
                seven_day_id = i;
                break;
            }
        }

        for i in day_id + 1 .. days.len() {
            if days[i] >= month_day {
                month_day_id = i;
                break;
            }
        }

        let mut bought_price = costs[0] + Self::backtrack(day_id + 1, days, costs, cache);
        bought_price = bought_price.min(costs[1] + Self::backtrack(seven_day_id, days, costs, cache));
        bought_price = bought_price.min(costs[2] + Self::backtrack(month_day_id, days, costs, cache));

        cache[day_id] = bought_price;
        cache[day_id] 
    }
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let mut cache = vec![-1; days.len()];
        Self::backtrack(0, &days, &costs, &mut cache)
    }
}