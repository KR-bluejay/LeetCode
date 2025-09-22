use std::collections:: {BTreeSet, BTreeMap, HashMap, HashSet  };
impl Solution {
    pub fn most_visited_pattern(
        usernames: Vec<String>, 
        timestamps: Vec<i32>, 
        websites: Vec<String>
    ) -> Vec<String> {
        let mut user_history_map: HashMap<&str, BTreeSet<(i32, &str)>> 
            = HashMap::with_capacity(usernames.len());
        let mut most_history_map: BTreeMap<Vec<&str>, HashSet<&str>> = BTreeMap::new();
        

        for id in 0 .. usernames.len() {
            let username = &usernames[id];
            let timestamp = &timestamps[id];
            let website = &websites[id];

            user_history_map.entry(username)
                .or_insert(BTreeSet::new())
                .insert((*timestamp, website));
        }


        for (user, user_history_set) in user_history_map.iter() {
            if user_history_set.len() < 3 {
                continue;
            }
            let user_history_list: Vec<&str> = user_history_set.iter().map(|v| v.1).collect();
            for i in 0 .. user_history_list.len() - 2 {
                let first = user_history_list[i];
                
                for j in i + 1 .. user_history_list.len() - 1 {
                    let second = user_history_list[j];

                    for k in j + 1 .. user_history_list.len() {
                        let third = user_history_list[k];

                        most_history_map.entry(vec![first, second, third])
                            .or_insert(HashSet::new())
                            .insert(user);
                    }
                }
            }
        }

        let mut result: Option<&Vec<&str>> = None;
        let mut result_count: usize = 0;

        for (k, v) in most_history_map.iter() {
            if result_count >= v.len() {
                continue;
            }
            result_count = v.len();
            result = Some(k);
        }
        // println!("{most_history_map:?}");
        result.unwrap().into_iter().map(|v| v.to_string()).collect()
    }
}