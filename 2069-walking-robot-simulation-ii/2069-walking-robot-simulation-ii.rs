struct Robot {
    width: i32,
    height: i32,
    cycle: i32,
    step: i32,
    moved: bool,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Robot {

    fn new(width: i32, height: i32) -> Self {
        let width = width - 1;
        let height = height - 1;

        Self {
            width,
            height,
            cycle: 2 * (width + height),
            step: 0,
            moved: false,
        }
    }
    
    fn step(&mut self, num: i32) {
        self.moved = true;
        self.step = (self.step + num) % self.cycle;
    }
    
    fn get_pos(&self) -> Vec<i32> {
        if self.step <= self.width {
            vec![self.step, 0]
        } else if self.step <= self.width + self.height {
            vec![self.width, self.step - self.width]
        } else if self.step <= 2 * self.width + self.height {
            vec![2 * self.width + self.height - self.step, self.height]
        } else {
            vec![0, self.cycle - self.step]
        }
    }
    
    fn get_dir(&self) -> String {
        if self.step == 0 && self.moved {
            String::from("South")
        } else if self.step <= self.width {
            String::from("East")
        } else if self.step <= self.width + self.height {
            String::from("North")
        } else if self.step <= 2 * self.width + self.height {
            String::from("West")
        } else {
            String::from("South")
        }
    }
}

/**
 * Your Robot object will be instantiated and called as such:
 * let obj = Robot::new(width, height);
 * obj.step(num);
 * let ret_2: Vec<i32> = obj.get_pos();
 * let ret_3: String = obj.get_dir();
 */