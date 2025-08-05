impl Solution {
    pub fn num_of_unplaced_fruits(
        mut fruits: Vec<i32>, 
        mut baskets: Vec<i32>
    ) -> i32 {
        let mut used_basket = vec![false; baskets.len()];

        for (fruit_id, fruit_item) in fruits.iter().enumerate() {
            for (basket_id, basket_item) in baskets.iter().enumerate() {
                if fruit_item <= basket_item && !used_basket[basket_id] {
                    used_basket[basket_id] = true;
                    break;
                }
            }
        }

        used_basket.iter().filter(|i| !*i).count() as i32
    }
}