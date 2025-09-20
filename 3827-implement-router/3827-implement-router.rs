use std::collections::{ VecDeque, HashSet, HashMap };
use std::cmp::Ordering;

struct Packet {
    source: i32,
    destination: i32,
    timestamp: i32,
}

struct Router {
    packets: VecDeque<Packet>,
    packet_set: HashSet<(i32, i32, i32)>,
    packet_dest: HashMap<i32, VecDeque<i32>>,
    limit: usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Router {

    fn new(memoryLimit: i32) -> Self {
        Self {
            // packets: VecDeque::with_capacity(memoryLimit as usize),
            // packet_set: HashSet::with_capacity(memoryLimit as usize),
            // packet_dest: HashMap::with_capacity(memoryLimit as usize),
            packets: VecDeque::with_capacity(memoryLimit.max(1000) as usize),
            packet_set: HashSet::with_capacity(memoryLimit.max(1000) as usize),
            packet_dest: HashMap::with_capacity(memoryLimit.max(1000) as usize),
            limit: memoryLimit as usize,
        }
    }
    
    fn add_packet(&mut self, source: i32, destination: i32, timestamp: i32) -> bool {
        if self.packet_set.contains(&(source, destination, timestamp)) {
            return false;
        }

        if self.packets.len() == self.limit {
            let Packet {destination, source, timestamp} = self.packets.pop_front().unwrap();
            
            self.packet_set.remove(&(source, destination, timestamp));
            self.packet_dest.get_mut(&destination).unwrap().pop_front();
        }


        self.packet_set.insert((source, destination, timestamp));
        self.packet_dest.entry(destination)
            .or_insert(VecDeque::new())
            .push_back(timestamp);
        self.packets.push_back(Packet {
            source,
            destination,
            timestamp,
        });

        true
    }
    
    fn forward_packet(&mut self) -> Vec<i32> {
        if let Some(pop_packet) = self.packets.pop_front() {
            self.packet_set.remove(&(pop_packet.source, pop_packet.destination, pop_packet.timestamp));
            self.packet_dest.get_mut(&pop_packet.destination).unwrap().pop_front();

            vec![pop_packet.source, pop_packet.destination, pop_packet.timestamp]
        } else {
            vec![]
        }
    }
    
    fn get_count(&self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        let mut count = 0;

        if let Some(packet_times) = self.packet_dest.get(&destination) {
            if packet_times.len() == 0 {
                return 0;
            }

            let temp_start_id = match packet_times.binary_search(&(start_time - 1)) {
                Ok(id) => id,
                Err(id) => id,
            };
            let mut start_id = 0;

            for id in temp_start_id .. packet_times.len() {
                if  packet_times[id] < start_time {
                    continue;
                } else if start_time <= packet_times[id] && packet_times[id] <= end_time {
                    start_id = id;
                    
                    break;
                } else if start_time > packet_times[id] || packet_times[id] > end_time {
                    return 0;
                }
            }

            let mut temp_end_id = match packet_times.binary_search(&(end_time + 1)) {
                Ok(id) => {
                    id.max(1)
                },
                Err(id) => {
                    id.max(1)
                },
            };
            temp_end_id = temp_end_id.min(packet_times.len() - 1);

            let mut end_id = 0;
            for id in (0 ..= temp_end_id).rev() {
                if  packet_times[id] > end_time {
                    continue;
                } else if start_time <= packet_times[id] && packet_times[id] <= end_time {
                    end_id = id;
                    
                    break;
                } else if start_time > packet_times[id] {
                    return 0;
                }
            }


            count = end_id - start_id + 1;
        }

        count as i32
    }
}

/**
 * Your Router object will be instantiated and called as such:
 * let obj = Router::new(memoryLimit);
 * let ret_1: bool = obj.add_packet(source, destination, timestamp);
 * let ret_2: Vec<i32> = obj.forward_packet();
 * let ret_3: i32 = obj.get_count(destination, startTime, endTime);
 */