use std::collections::{ BinaryHeap };
use std::cmp::{ Ordering };

#[derive(Eq, PartialEq)]
struct RoomTime {
    total_time: i32,
    is_one_move: bool,
    row: usize,
    col: usize,
}

impl Ord for RoomTime {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total_time.cmp(&other.total_time).reverse()
    }
}

impl PartialOrd for RoomTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        let mut room_record: Vec<Vec<i32>> = vec![vec![i32::MAX; move_time[0].len()]; move_time.len()];
        let mut room_queue: BinaryHeap<RoomTime> = BinaryHeap::with_capacity(move_time.len() * move_time[0].len());

        room_queue.push(RoomTime {
            row: 0,
            col: 0,
            is_one_move: true,
            total_time: 0
        });

        while !room_queue.is_empty() {
            let RoomTime { row, col, is_one_move, total_time } = room_queue.pop().unwrap();
            let next_time: i32 = if is_one_move {
                1
            } else {
                2
            };

            if room_record[row][col] < total_time {
                continue;
            }

            if row > 0 && room_record[row - 1][col] > move_time[row - 1][col].max(total_time) + next_time {
                room_record[row - 1][col] =  move_time[row - 1][col].max(total_time) + next_time;
                room_queue.push(RoomTime {
                    row: row - 1,
                    col: col,
                    is_one_move: !is_one_move,
                    total_time: room_record[row - 1][col]
                });
            }

            if row + 1 < room_record.len() 
            && room_record[row + 1][col] > move_time[row + 1][col].max(total_time) + next_time {
                room_record[row + 1][col] =  move_time[row + 1][col].max(total_time) + next_time;
                room_queue.push(RoomTime {
                    row: row + 1,
                    col: col,
                    is_one_move: !is_one_move,
                    total_time: room_record[row + 1][col]
                });
            }

            if col > 0 && room_record[row][col - 1] > move_time[row][col - 1].max(total_time) + next_time {
                room_record[row][col - 1] =  move_time[row][col - 1].max(total_time) + next_time;
                room_queue.push(RoomTime {
                    row: row,
                    col: col - 1,
                    is_one_move: !is_one_move,
                    total_time: room_record[row][col - 1]
                });
            }

            if col + 1 < room_record[0].len() 
            && room_record[row][col + 1] > move_time[row][col + 1].max(total_time) + next_time {
                room_record[row][col + 1] =  move_time[row][col + 1].max(total_time) + next_time;
                room_queue.push(RoomTime {
                    row: row,
                    col: col + 1,
                    is_one_move: !is_one_move,
                    total_time: room_record[row][col + 1]
                });
            }
        }

        room_record[room_record.len() - 1][room_record[0].len() - 1]
    }
}