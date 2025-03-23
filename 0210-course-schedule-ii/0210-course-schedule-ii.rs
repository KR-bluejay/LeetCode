use std::collections::{ HashMap, HashSet };

impl Solution {
    fn take_course(course_map: &HashMap<usize, HashSet<usize>>, course_require_map: &HashMap<usize, HashSet<usize>>, course_history: &mut Vec<i32>, visited: &mut Vec<bool>, course_id: usize) {
        let required_course_set = match course_require_map.get(&course_id) {
            Some(s) => s,
            None => return
        };

        if !required_course_set.iter().all(|course_id| visited[*course_id]) {
            return;
        }
        visited[course_id] = true;
        course_history.push(course_id as i32);


        let unlock_course_set = match course_map.get(&course_id) {
            Some(s) => s,
            None => return
        };


        for unlock_course_id in unlock_course_set.iter() {
            if visited[*unlock_course_id] {
                continue;
            }
            Self::take_course(course_map, course_require_map, course_history, visited, *unlock_course_id);
        }
    }

    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut course_map: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut course_require_map: HashMap<usize, HashSet<usize>> = HashMap::new();

        let mut visited: Vec<bool> = vec![false; num_courses as usize];
        let mut course_history: Vec<i32> = Vec::with_capacity(num_courses as usize);

        for prerequisite_item in prerequisites.iter() {
            let course_id = prerequisite_item[0] as usize;
            let required_course_id = prerequisite_item[1] as usize;


            course_map.entry(required_course_id).or_insert(HashSet::new()).insert(course_id);
            course_require_map.entry(course_id).or_insert(HashSet::new()).insert(required_course_id);
        }

        for i in 0 .. num_courses {
            if course_require_map.get(&(i as usize)).is_none() {
                course_history.push(i);
                visited[i as usize] = true;
            }
        }

        for i in 0 .. num_courses as usize {
            if visited[i] {
                continue;
            }
            Self::take_course(&course_map, &course_require_map, &mut course_history, &mut visited, i);
        }

        if course_history.len() != visited.len() {
            return Vec::new();
        }
        course_history
    }
}