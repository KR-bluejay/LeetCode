struct Bank {
    balance: Vec<i64>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Bank {

    fn new(balance: Vec<i64>) -> Self {
        Bank {
            balance
        }
    }
    
    #[inline(always)]
    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        let account1 = account1 as usize - 1;
        let account2 = account2 as usize - 1;

        if account1 >= self.balance.len() 
        || account2 >= self.balance.len() 
        || self.balance[account1] < money {
            return false;
        }

        self.balance[account1] -= money;
        self.balance[account2] += money;

        true
    }
    
    #[inline(always)]
    fn deposit(&mut self, account: i32, money: i64) -> bool {
        let account = account as usize - 1;

        if account >= self.balance.len() {
            return false;
        }
        
        self.balance[account] += money;

        true
    }
    
    #[inline(always)]
    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        let account = account as usize - 1;
        
        if account >= self.balance.len() || self.balance[account] < money {
            return false;
        }
        self.balance[account] -= money;

        true
    }
}

/**
 * Your Bank object will be instantiated and called as such:
 * let obj = Bank::new(balance);
 * let ret_1: bool = obj.transfer(account1, account2, money);
 * let ret_2: bool = obj.deposit(account, money);
 * let ret_3: bool = obj.withdraw(account, money);
 */