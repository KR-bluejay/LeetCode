struct StockSpanner {
    records: Vec<(i32, i32)>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {

    fn new() -> Self {
        Self {
            records: Vec::with_capacity(10000),
        }
    }
    
    fn next(&mut self, price: i32) -> i32 {
        let mut day = 1;

        while let Some(&(last_price, last_day)) = self.records.last() 
        && last_price <= price {
            day += last_day;
            
            self.records.pop();
        }

        self.records.push((price, day));

        day
    }
}

/**
 * Your StockSpanner object will be instantiated and called as such:
 * let obj = StockSpanner::new();
 * let ret_1: i32 = obj.next(price);
 */