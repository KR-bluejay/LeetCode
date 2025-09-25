use std::collections::HashMap;

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        let mut numerator = numerator as i64;
        let mut denominator = denominator as i64;

        if numerator < 0 && denominator < 0 {
            numerator = numerator.abs();
            denominator = denominator.abs();
        }

        let quotient = numerator / denominator;
        let remaining = numerator % denominator;

        if remaining == 0 {
            return quotient.to_string();
        }


        let mut fract_map: HashMap<(i64, i64), usize> = HashMap::new();
        let mut fract_list: Vec<i64> = Vec::with_capacity(100);
        let is_neg = (numerator < 0) ^ (denominator < 0);

        let mut int_part = quotient.clone().abs();

        numerator = (remaining.clone() * 10).abs();
        denominator = denominator.abs();
         

        let mut fract_id = 0;
        let mut similar_count = usize::MAX;
        let mut similiar_start_id = usize::MAX;
        let mut similar_dist = usize::MAX;

        while numerator != 0 {
            let quotient = numerator / denominator;
            let remaining = numerator % denominator;

            // println!("{quotient} {remaining} {is_neg}");

            fract_list.push(quotient);
            
            if let Some(similar_id) = fract_map.get(&(quotient, remaining)) {
                if similar_count == usize::MAX || similar_dist == usize::MAX {
                    similar_dist = (fract_id - similar_id);
                    similar_count = 1;
                    similiar_start_id = *similar_id;
                } else if fract_id - similar_id == similar_dist {
                    similar_count += 1;
                } else {
                    similar_count = usize::MAX;
                    similar_dist = usize::MAX;
                    similiar_start_id = fract_id;
                }
            } else {
                similar_count = usize::MAX;
                similar_dist = usize::MAX;
                similiar_start_id = fract_id;
            }

            
            if similar_dist != usize::MAX && similar_count == similar_dist {
                break;
            }

            fract_map.insert((quotient, remaining), fract_id);

            numerator = remaining.clone() * 10;
            fract_id += 1;
        }

        let mut rtn_str: String = if is_neg {format!("-{int_part}.") } else { format!("{int_part}.") };

        // println!("{fract_list:?}");
        // println!("{similar_dist} {similiar_start_id}");

        for (id, num) in fract_list.iter().enumerate() {
            if id == similiar_start_id && similar_dist != usize::MAX {
                rtn_str.push('(');
                rtn_str.push(char::from_digit(*num as u32, 10).unwrap_or('0'));
                
                if similar_dist == 1 {
                    rtn_str.push(')');

                    return rtn_str; 
                }
            } else if id == similiar_start_id + similar_dist - 1 && similar_dist != usize::MAX  {
                rtn_str.push(char::from_digit(*num as u32, 10).unwrap_or('0'));
                rtn_str.push(')');

                return rtn_str;
            } else {
                rtn_str.push(char::from_digit(*num as u32, 10).unwrap_or('0'));
            }
        }

        rtn_str
    }
}