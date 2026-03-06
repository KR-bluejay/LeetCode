use std::collections::HashMap;

struct Node {
    key: i32,
    value: i32,
    prev: *mut Node,
    next: *mut Node,
}

impl Node {
    fn new(key: i32, value: i32) -> *mut Node {
        let boxed_node = Box::new(Self {
            key, 
            value, 
            prev: std::ptr::null_mut(),
            next: std::ptr::null_mut(),
        });

        Box::into_raw(boxed_node)
    }
}

struct LRUCache {
    cache_cap: usize,
    cache_map: HashMap<i32, *mut Node>,
    head: *mut Node,
    tail: *mut Node,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {

    fn new(capacity: i32) -> Self {
        let cache_cap = capacity as usize;
        let cache_map = HashMap::with_capacity(cache_cap);

        let head = Node::new(0, 0);
        let tail = Node::new(0, 0);

        unsafe {
            (*head).next = tail;
            (*tail).prev = head;
        }

        Self {
            cache_cap,
            cache_map,
            head,
            tail,
        }
    }

    fn add_node(&mut self, node: *mut Node) {
        unsafe {
            (*node).prev = self.head;
            (*node).next = (*self.head).next;
    
            (*(*self.head).next).prev = node;
            (*self.head).next = node;
        }
    }

    fn remove_node(&mut self, node: *mut Node) {
        unsafe {
            let prev = (*node).prev;
            let next = (*node).next;
            
            (*prev).next = next;
            (*next).prev = prev;
        }
    }
    
    fn move_to_head(&mut self, node: *mut Node) {
        unsafe {
            self.remove_node(node);
            self.add_node(node);
        }
    }
    
    fn get(&mut self, key: i32) -> i32 {
        if let Some(&node_ptr) = self.cache_map.get(&key) {
            unsafe {
                self.move_to_head(node_ptr);
                (*node_ptr).value
            }
        } else {
            -1
        }
    }

    fn pop_tail(&mut self) {
        let last_node = unsafe {(*self.tail).prev};
        
        self.remove_node(last_node);
        unsafe {
            self.cache_map.remove(&(*last_node).key);
            let _ = Box::from_raw(last_node);
        }
    }
    
    fn put(&mut self, key: i32, value: i32) {
        if let Some(&node_ptr) = self.cache_map.get(&key) {
            unsafe {
                (*node_ptr).value = value;
                self.move_to_head(node_ptr)
            }
        } else {
            if self.cache_map.len() == self.cache_cap {
                unsafe {
                    self.pop_tail();
                }
            }

            let new_node = Node::new(key, value);
            self.add_node(new_node);
            self.cache_map.insert(key, new_node);
        }
    }
}

impl Drop for LRUCache {
    fn drop(&mut self) {
        let mut curr = self.head;
        while !curr.is_null() {
            unsafe {
                let next = (*curr).next;
                let _ = Box::from_raw(curr);
                curr = next;
            }
        }
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */