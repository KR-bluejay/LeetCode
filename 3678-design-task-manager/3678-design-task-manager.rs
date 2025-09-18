use std::collections::{ BinaryHeap, HashMap };

struct TaskManager {
    task_to_user: HashMap<i32, (i32, i32)>,
    priority_heap: BinaryHeap<(i32, i32)>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TaskManager {

    fn new(tasks: Vec<Vec<i32>>) -> Self {
        let mut task_to_user: HashMap<i32, (i32, i32)> = HashMap::with_capacity(tasks.len());
        let mut priority_heap: BinaryHeap<(i32, i32)> = BinaryHeap::with_capacity(tasks.len());

        for task in tasks {
            let [user_id, task_id, priority]: [i32; 3] = task.try_into().unwrap();

            task_to_user.insert(task_id, (user_id, priority));
            priority_heap.push((priority, task_id));
        }

        Self {
            task_to_user,
            priority_heap
        }
    }
    
    fn add(&mut self, user_id: i32, task_id: i32, priority: i32) {
        self.task_to_user.insert(task_id, (user_id, priority));
        self.priority_heap.push((priority, task_id));
    }
    
    fn edit(&mut self, task_id: i32, new_priority: i32) {
        self.task_to_user.entry(task_id).and_modify(|v| (*v).1 = new_priority);
        self.priority_heap.push((new_priority, task_id));
    }
    
    fn rmv(&mut self, task_id: i32) {
        self.task_to_user.remove(&task_id);
    }
    
    fn exec_top(&mut self) -> i32 {
        while let Some((priority, task_id)) = self.priority_heap.pop() {
            if let Some(&(task_user_id, task_priority)) = self.task_to_user.get(&task_id) {
                if priority == task_priority {
                    self.task_to_user.remove(&task_id);
        
                    return task_user_id;
                }
            }
        }

        -1
    }
}

/**
 * Your TaskManager object will be instantiated and called as such:
 * let obj = TaskManager::new(tasks);
 * obj.add(userId, taskId, priority);
 * obj.edit(taskId, newPriority);
 * obj.rmv(taskId);
 * let ret_4: i32 = obj.exec_top();
 */