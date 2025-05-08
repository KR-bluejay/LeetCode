use std::collections::{ HashMap, HashSet };

struct Logger {
    message_map: HashMap<String, i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Logger {

    fn new() -> Self {
        Logger {
            message_map: HashMap::new(),
        }
    }
    
    fn should_print_message(&mut self, timestamp: i32, message: String) -> bool {
        if let Some(message_timestamp) = self.message_map.get_mut(&message) {
            if timestamp < *message_timestamp {
                return false;
            }
            *message_timestamp = timestamp + 10;
        } else {
            self.message_map.insert(message, timestamp + 10);
        }

        true
    }
}

/**
 * Your Logger object will be instantiated and called as such:
 * let obj = Logger::new();
 * let ret_1: bool = obj.should_print_message(timestamp, message);
 */