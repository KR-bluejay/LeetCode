use std::collections::{ BTreeMap, HashMap, BinaryHeap };

struct TaskManager {
    task_user_map: HashMap<i32, i32>,
    task_priority_map: HashMap<i32, i32>,
    priority_tasks_map: BTreeMap<i32, BinaryHeap<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TaskManager {
    fn new(tasks: Vec<Vec<i32>>) -> Self {
        let mut task_priority_map: HashMap<i32, i32> = HashMap::with_capacity(tasks.len());
        let mut task_user_map: HashMap<i32, i32> = HashMap::with_capacity(tasks.len());
        let mut priority_tasks_map: BTreeMap<i32, BinaryHeap<i32>> = BTreeMap::new();

        for task in tasks.iter() {
            let user_id = task[0];
            let task_id = task[1];
            let priority = task[2];

            task_user_map.insert(task_id.clone(), user_id);
            task_priority_map.insert(task_id.clone(), priority.clone() * -1);
            priority_tasks_map.entry(priority * -1)
                .or_insert(BinaryHeap::new())
                .push(task_id);
        }

        Self {
            task_priority_map,
            task_user_map,
            priority_tasks_map
        }
    }
    
    fn add(&mut self, user_id: i32, task_id: i32, priority: i32) {
        self.task_priority_map.insert(task_id.clone(), priority.clone() * -1);
        self.task_user_map.insert(task_id.clone(), user_id);
        self.priority_tasks_map.entry(priority.clone() * -1)
            .or_insert(BinaryHeap::new())
            .push(task_id);
    }
    
    fn edit(&mut self, task_id: i32, new_priority: i32) {
        self.priority_tasks_map.entry(new_priority * -1)
            .or_insert(BinaryHeap::new())
            .push(task_id);
        self.task_priority_map.entry(task_id).and_modify(|v| {
            *v = new_priority * -1
        });
    }
    
    fn rmv(&mut self, task_id: i32) {
        self.task_priority_map.remove(&task_id);
        self.task_user_map.remove(&task_id);
    }
    
    fn exec_top(&mut self) -> i32 {
        let mut top_user_id = -1;

        self.priority_tasks_map.retain(|_, v| !v.is_empty());
        
        for (k, v) in self.priority_tasks_map.iter_mut() {
            while let Some(task_id) = v.pop() {
                if let Some(priority) = self.task_priority_map.get(&task_id) {
                    if k == priority {
                        self.task_priority_map.remove(&task_id);
                        top_user_id =  self.task_user_map.remove(&task_id).unwrap();
                        break;
                    }
                }
            }

            if top_user_id != -1 {
                break;
            }
        }
        

        return top_user_id;
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