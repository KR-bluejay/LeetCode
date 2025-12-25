impl Solution {
    fn combine(
        budget: usize, 
        dp1: &Vec<i32>, 
        dp2: &Vec<i32>
    ) 
    -> Vec<i32> {
        let mut new_dp: Vec<i32> = vec![0; budget + 1];

        for i in 0 ..= budget {
            for j in 0 ..= (budget - i) {
                new_dp[i + j] = new_dp[i + j].max(dp1[i] + dp2[j]);
            }
        }

        new_dp
    }
    fn get_profit(
        id: usize, 
        budget: usize,
        graph: &Vec<Vec<usize>>, 
        present: &Vec<i32>,
        future: &Vec<i32>,
    ) -> (Vec<i32>, Vec<i32>) {
        let normal_price = (present[id]) as i32;
        let discount_price = (present[id] / 2) as i32;

        let normal_future = future[id] - normal_price;
        let discount_future = future[id] - discount_price;

        let mut child_profits: Vec<(Vec<i32>, Vec<i32>)> = Vec::with_capacity(budget + 1);

        for &child_id in graph[id].iter() {
            child_profits.push(
                Self::get_profit(
                    child_id, 
                    budget, 
                    graph,
                    present,
                    future
                )
            );
        }


        let mut normal_profits = vec![0; budget + 1];
        let mut discount_profits = vec![0; budget + 1];


        for child_profit in child_profits {
            let child_normal = child_profit.0;
            let child_discount = child_profit.1;

            normal_profits = Self::combine(
                budget, 
                &normal_profits,
                &child_normal, 
            );
            discount_profits = Self::combine(
                budget, 
                &discount_profits, 
                &child_discount
            );
        }


        let mut dp_normal = vec![0; budget + 1];
        let mut dp_discount = vec![0; budget + 1]; normal_profits.clone();

        for i in 0 ..= budget {
            dp_normal[i] = normal_profits[i];
            dp_discount[i] = normal_profits[i];
        }

        for i in normal_price as usize ..= budget {
            let new_profit = discount_profits[i - normal_price as usize] + normal_future;

            dp_normal[i] = dp_normal[i].max(new_profit);
        }


        for i in discount_price as usize ..= budget {
            let new_profit = discount_profits[i - discount_price as usize] + discount_future;

            dp_discount[i] = dp_discount[i].max(new_profit);
        }


        (dp_normal, dp_discount)
    }
    pub fn max_profit(
        n: i32, 
        present: Vec<i32>, 
        future: Vec<i32>, 
        hierarchies: Vec<Vec<i32>>, 
        budget: i32
    ) -> i32 {
        let staff_count = n as usize;
        let budget = budget as usize;
        let mut graph: Vec<Vec<usize>> = vec![Vec::with_capacity(staff_count); staff_count];

        for hierarchy in hierarchies {
            let (boss_id, staff_id) = (hierarchy[0] as usize, hierarchy[1] as usize);

            graph[boss_id - 1].push(staff_id - 1);
        }

        let mut max_profit = 0;
        let dp = Self::get_profit(0, budget, &graph, &present, &future);


        for id in 0 ..= budget {
            max_profit = max_profit.max(dp.0[id]);
        }

        max_profit
    }
}