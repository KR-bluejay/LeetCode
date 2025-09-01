use std::cmp::{Ordering};
use std::collections::{BinaryHeap};

#[derive(Clone, Debug)]
struct ClassRatio {
    total: i32,
    pass: i32,
    ratio: f64,
}

impl PartialEq for ClassRatio {
    fn eq(&self, other: &Self) -> bool {
        self.total == other.total
            && self.pass == other.pass
            && self.ratio.to_bits() == other.ratio.to_bits()
    }
}
impl Eq for ClassRatio {}


impl Ord for ClassRatio {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ratio
            .total_cmp(&other.ratio)
            .then_with(|| self.pass.cmp(&other.pass))
            .then_with(|| self.total.cmp(&other.total))
    }
}

impl PartialOrd for ClassRatio {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, mut extra_students: i32) -> f64 {
        let mut class_ratios: BinaryHeap<ClassRatio> = classes.into_iter().map(|v| ClassRatio {
            pass: v[0],
            total: v[1],
            ratio: ((v[0] + 1) as f64 / ((v[1] + 1) as f64)) - (v[0] as f64 / v[1] as f64)
        }).collect();
        
        while let Some(class_item) = class_ratios.pop() {
            if extra_students <= 0 {
                class_ratios.push(class_item);
                break;
            }

            let pass = class_item.pass + 1;
            let total = class_item.total + 1;
            let ratio = ((pass + 1) as f64 / ((total + 1) as f64)) - (pass as f64 / total as f64);
            
            extra_students -= 1;

            class_ratios.push(ClassRatio {
                pass,
                total, 
                ratio
            });
        }

        println!("{class_ratios:?}");

        class_ratios.iter().map(|v| v.pass as f64 / v.total as f64).sum::<f64>() / class_ratios.len() as f64
    }
}