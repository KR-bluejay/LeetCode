use std::collections::{ BinaryHeap };
use std::cmp::{ Ordering };

#[derive(Eq, PartialEq)]
struct RoomTime {
    time: i32,
    row: usize,
    col: usize,
}

impl Ord for RoomTime {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time).reverse()
    }
}

impl PartialOrd for RoomTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl Solution {
    pub fn min_time_to_reach(
        move_time: Vec<Vec<i32>>
    ) -> i32 {
        let mut room_record: Vec<Vec<i32>> = vec![vec![i32::MAX; move_time[0].len()]; move_time.len()];
        let mut min_heap: BinaryHeap<RoomTime> = BinaryHeap::with_capacity(move_time.len() * move_time[0].len());

        min_heap.push(RoomTime {
            time: 0,
            row: 0,
            col: 0,
        });

        while !min_heap.is_empty() {
            let RoomTime {time, row, col} = min_heap.pop().unwrap();

            if room_record[row][col] < time {
                continue;
            }

            if row > 0 && room_record[row - 1][col] > move_time[row - 1][col].max(time) + 1 {
                room_record[row - 1][col] = move_time[row - 1][col].max(time) + 1;

                min_heap.push(RoomTime {
                    time: move_time[row - 1][col].max(time) + 1,
                    row: row - 1,
                    col: col
                });
            }

            if row + 1 < room_record.len() && room_record[row + 1][col] > move_time[row + 1][col].max(time) + 1 {
                room_record[row + 1][col] = move_time[row + 1][col].max(time) + 1;

                min_heap.push(RoomTime {
                    time: move_time[row + 1][col].max(time) + 1,
                    row: row + 1,
                    col: col,
                });
            }

             if col > 0 && room_record[row][col - 1] > move_time[row][col - 1].max(time) + 1 {
                room_record[row][col - 1] = move_time[row][col - 1].max(time) + 1;

                min_heap.push(RoomTime {
                    time: move_time[row][col - 1].max(time) + 1,
                    row: row,
                    col: col - 1
                });
            }

            if col + 1 < room_record[0].len() && room_record[row][col + 1] > move_time[row][col + 1].max(time) + 1 {
                room_record[row][col + 1] = move_time[row][col + 1].max(time) + 1;

                min_heap.push(RoomTime {
                    time: move_time[row][col + 1].max(time) + 1,
                    row: row,
                    col: col + 1,
                });
            }
        }

        
        room_record[move_time.len() - 1][move_time[0].len() - 1]
    }
}