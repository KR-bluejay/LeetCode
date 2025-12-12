use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum EventType {
    OFFLINE = 0,
    MESSAGE = 1,
}

impl Solution {
    pub fn count_mentions(number_of_users: i32, events: Vec<Vec<String>>) -> Vec<i32> {
        let number_of_users = number_of_users as usize;

        let mut user_status: Vec<u32> = vec![0; number_of_users];
        let mut user_mention: Vec<i32> = vec![0; number_of_users];

        let mut all_count = 0;
        let mut last_msg_id = 0;

        let mut msg_map: BTreeMap<(u32, EventType), Vec<usize>> = BTreeMap::new();

        for event in events {
            let event_type: EventType = match event[0].as_str() {
                "OFFLINE" => EventType::OFFLINE,
                "MESSAGE" => EventType::MESSAGE,
                _ => unreachable!(),
            };
            
            let event_timestamp = event[1].parse::<u32>().unwrap();
            let mentions: Vec<usize> = event[2].split_whitespace().map(|v| {
                match v {
                    "ALL" => usize::MAX,
                    "HERE" => usize::MAX - 1,
                    _ => {
                        let v_str = if v.contains("id") {
                            v.split_at(2).1
                        } else {
                            v
                        };

                        v_str.parse::<usize>().unwrap()
                    }
                }
            }).collect();

            if event_type == EventType::MESSAGE {
                last_msg_id = last_msg_id.max(event_timestamp);
            }

            msg_map.entry((event_timestamp, event_type))
                .and_modify(|v| v.extend(&mentions))
                .or_insert(mentions);

        }

        for (msg_key, msg_target) in msg_map.iter() {
            let msg_timestamp = msg_key.0;
            let msg_type = msg_key.1;

            if msg_timestamp > last_msg_id {
                break;
            }

            match msg_type {
                EventType::OFFLINE => {
                    for &id in msg_target.iter() {
                        user_status[id] = user_status[id].max(msg_timestamp + 60);
                    }
                },
                EventType::MESSAGE => {
                    for &id in msg_target.iter() {
                        if id == usize::MAX {
                            all_count += 1;
                        } else if id == usize::MAX - 1 {
                            for (user_id, &status) in  user_status.iter().enumerate() {
                                user_mention[user_id] += (status <= msg_timestamp ) as i32;
                            }
                        } else {
                            user_mention[id] += 1;
                        }
                    }
                }

            }
        }


        user_mention.into_iter().map(|mut v| {
            v += all_count;
            v
        }).collect()
    }
}