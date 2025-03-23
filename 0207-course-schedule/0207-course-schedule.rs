use std::collections::{ HashMap, HashSet };

impl Solution {
    fn take_course(
        course_map: &HashMap<usize, HashSet<usize>>, 
        course_require_map: &HashMap<usize, HashSet<usize>>, 
        course_history: &mut Vec<bool>, 
        course_id: usize
    ) {
        if let Some(reuired_course_set) = course_require_map.get(&course_id) {
            if !reuired_course_set.iter().all(|re_course_id| course_history[*re_course_id]) {
                return;
            }
        }

        course_history[course_id] = true;

        let after_course_set = match course_map.get(&course_id) {
            Some(r) => r,
            None => return 
        };


        for after_course_id in after_course_set.iter() {
            if course_history[*after_course_id] {
                continue;
            }
            Self::take_course(course_map, course_require_map, course_history, *after_course_id);
        }
    }
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut course_require_map: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut course_map: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut is_advanced_course: Vec<bool> = vec![false; num_courses as usize];
        let mut course_history: Vec<bool> = vec![false; num_courses as usize];


        for prerequisite_item in prerequisites.iter() {
            let course_id = prerequisite_item[0] as usize;
            let pre_course_id = prerequisite_item[1] as usize;
            is_advanced_course[course_id] = true;

            if !course_require_map.get(&pre_course_id).iter()
                .all(|pre_course_set| !pre_course_set.contains(&course_id)) {
                return false;
            }

            course_require_map.entry(course_id).or_insert(HashSet::new()).insert(pre_course_id);
            course_map.entry(pre_course_id).or_insert(HashSet::new()).insert(course_id);
        }

        for i in 0 .. is_advanced_course.len() {
            if is_advanced_course[i] || course_history[i] {
                continue;
            }

            if course_map.get(&i).is_none() {
                course_history[i] = true;

                continue;
            }
            Self::take_course(&course_map, &course_require_map, &mut course_history, i);
        }
        course_history.iter().all(|course| *course == true)
    }
}