struct StockSpanner {
    records: Vec<u32>,
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
        let price = price as u32;
        let mut day = 1;

        while let Some(&packed) = self.records.last() {
            let last_price = (packed >> 15);
            let last_day = packed & 0x7FFF;

            if last_price <= price {
                day += last_day;

                self.records.pop();
            } else {
                break;
            }
        }

        self.records.push((price << 15) | day);

        day as i32
    }
}

/**
 * Your StockSpanner object will be instantiated and called as such:
 * let obj = StockSpanner::new();
 * let ret_1: i32 = obj.next(price);
 */