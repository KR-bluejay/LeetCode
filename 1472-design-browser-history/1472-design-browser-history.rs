struct BrowserHistory {
    forward: Vec<String>,
    backward: Vec<String>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BrowserHistory {

    fn new(homepage: String) -> Self {
        Self {
            forward: Vec::new(),
            backward: vec![homepage],
        }
    }
    
    fn visit(&mut self, url: String) {
        self.forward.clear();
        self.backward.push(url);
    }
    
    fn back(&mut self, steps: i32) -> String {
        let steps = steps as usize;
        
        for _ in 0 .. (self.backward.len().max(1) - 1).min(steps) {
            self.forward.push(self.backward.pop().unwrap());
        }

        self.backward.last().unwrap().clone()
    }
    
    fn forward(&mut self, steps: i32) -> String {
        let steps = steps as usize;
        
        for _ in 0 .. self.forward.len().min(steps) {
            self.backward.push(self.forward.pop().unwrap());
        }

        self.backward.last().unwrap().clone()
    }
}

/**
 * Your BrowserHistory object will be instantiated and called as such:
 * let obj = BrowserHistory::new(homepage);
 * obj.visit(url);
 * let ret_2: String = obj.back(steps);
 * let ret_3: String = obj.forward(steps);
 */